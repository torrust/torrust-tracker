/*
 *	Copyright © 2012-2017 Naim A.
 *
 *	This file is part of UDPT.
 *
 *		UDPT is free software: you can redistribute it and/or modify
 *		it under the terms of the GNU General Public License as published by
 *		the Free Software Foundation, either version 3 of the License, or
 *		(at your option) any later version.
 *
 *		UDPT is distributed in the hope that it will be useful,
 *		but WITHOUT ANY WARRANTY; without even the implied warranty of
 *		MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *		GNU General Public License for more details.
 *
 *		You should have received a copy of the GNU General Public License
 *		along with UDPT.  If not, see <http://www.gnu.org/licenses/>.
 */
#include <event2/buffer.h>
#include <event2/event.h>
#include <event2/thread.h>
#include <cstdint>
#include "WebApp.hpp"
#include "logging.hpp"
#include "db/driver_sqlite.hpp"
#include "tools.h"


namespace UDPT {
    WebApp::WebApp(UDPT::Data::DatabaseDriver &db,
                   const std::string& listenIP,
                   uint16_t listenPort): m_db(db),
                                         m_listenIP(listenIP),
                                         m_listenPort(listenPort) {

#ifdef WIN32
        ::evthread_use_windows_threads();
#elif defined(__linux__) || defined(__FreeBSD__)
        ::evthread_use_pthreads();
#else
#error evthread threading required, no compatible library was found.
#endif

        m_eventBase = std::shared_ptr<struct event_base>(::event_base_new(), ::event_base_free);
        if (nullptr == m_eventBase) {
            LOG_ERR("webapp", "Failed to create event base");
            throw std::exception();
        }

        m_httpServer = std::shared_ptr<struct evhttp>(::evhttp_new(m_eventBase.get()), ::evhttp_free);
        if (nullptr == m_httpServer) {
            LOG_ERR("webapp", "Failed to create http base");
            throw std::exception();
        }

        if (0 != ::evhttp_bind_socket(m_httpServer.get(), m_listenIP.c_str(), m_listenPort)) {
            LOG_ERR("webapp", "Failed to bind socket");
            throw std::exception();
        }

        LOG_INFO("webapp", "HTTP server bound to " << m_listenIP.c_str() << ":" << m_listenPort);

        ::evhttp_set_allowed_methods(m_httpServer.get(), EVHTTP_REQ_GET | EVHTTP_REQ_POST | EVHTTP_REQ_DELETE);

        ::evhttp_set_gencb(m_httpServer.get(), viewNotFound, this);
        ::evhttp_set_cb(m_httpServer.get(), "/", [](struct evhttp_request *req, void *){
            setCommonHeaders(req);
            sendReply(req, 200, "OK", HOME_PAGE);
        }, this);
        ::evhttp_set_cb(m_httpServer.get(), "/announce", [](struct evhttp_request *req, void *){
            setCommonHeaders(req);
            sendReply(req, 200, "OK", ANNOUNCE_PAGE);
        }, this);
        ::evhttp_set_cb(m_httpServer.get(), "/api/torrents", viewApiTorrents, this);
    }

    WebApp::~WebApp() {
        try {
            if (m_workerThread.joinable()) {
                m_workerThread.join();
            }
        } catch (...) {
            LOG_FATAL("webapp", "exception thrown @ WebApp termination.");
        }
    }

    void WebApp::start() {
        LOG_INFO("webapp", "Starting WebApp");
        LOG_INFO("webapp", "compiled with libevent " << LIBEVENT_VERSION << ", running with " << event_get_version());
        m_workerThread = std::thread(workerThread, this);
    }

    void WebApp::stop() {
        if (!m_isRunning) {
            return;
        }
        m_isRunning = false;

        LOG_INFO("webapp", "Requesting WebApp to stop");
        ::event_base_loopbreak(m_eventBase.get());
    }

    const std::string WebApp::ANNOUNCE_PAGE = "d14:failure reason41:udpt: This is a udp tracker, not HTTP(s).e";
    const std::string WebApp::NOT_FOUND_PAGE = "<h2>Not Found</h2>";
    const std::string WebApp::HOME_PAGE = "<html>"
            "<head>"
            "<title>UDPT</title>"
            "</head>"
            "<body>"
            "<h2>UDPT Tracker</h2>"
            "<div style=\"text-align: center; font-size: small;\"><a href=\"https://github.com/naim94a/udpt\">https://github.com/naim94a/udpt</a></div>"
            "</body>"
            "</html>";
    const std::string WebApp::JSON_INVALID_METHOD = "{\"error\": \"Invalid method\"}";
    const std::string WebApp::JSON_INTERNAL_ERROR = "{\"error\": \"Internal Server Error\"}";
    const std::string WebApp::JSON_PARAMS_REQUIRED = "{\"error\": \"This method requires parameters.\"}";
    const std::string WebApp::JSON_INFOHASH_REQUIRED = "{\"error\": \"exactly one info_hash argument is required.\"}";
    const std::string WebApp::JSON_INFOHASH_INVALID = "{\"error\": \"info_hash length is incorrect.\"}";
    const std::string WebApp::JSON_TORRENT_ADD_FAIL = "{\"error\": \"Failed to add torrent.\"}";
    const std::string WebApp::JSON_TORRENT_REMOVE_FAIL = "{\"error\": \"Failed to remove torrent.\"}";
    const std::string WebApp::JSON_OKAY = "{\"result\": \"Okay\"}";
    const std::string WebApp::JSON_OKAY_DYNAMIC = "{\"result\": \"Okay\", \"note\": \"tracker is in dynamic mode.\"}";


    void WebApp::workerThread(WebApp *app) {
        app->m_isRunning = true;
        ::event_base_dispatch(app->m_eventBase.get());

        LOG_INFO("webapp", "Worker " << std::this_thread::get_id() << " exited");
    }

    void WebApp::viewApiTorrents(struct ::evhttp_request *req, void *app_) {
        WebApp *app = reinterpret_cast<WebApp*>(app_);

        setCommonHeaders(req);
        addHeaders(req, std::multimap<std::string, std::string>( { {"Content-Type", "text/json"} } ));

        enum evhttp_cmd_type requestMethod = ::evhttp_request_get_command(req);
        if (requestMethod != EVHTTP_REQ_POST && requestMethod != EVHTTP_REQ_DELETE) {
            sendReply(req, HTTP_BADMETHOD, "Bad Method", JSON_INVALID_METHOD);
            return;
        }

        const struct evhttp_uri *requestUri = ::evhttp_request_get_evhttp_uri(req);
        if (nullptr == requestUri) {
            LOG_ERR("webapp", "evhttp_request_get_evhttp_uri() returned NULL.");
            sendReply(req, HTTP_INTERNAL, "Internal Server Error", JSON_INTERNAL_ERROR);
            return;
        }

        const char *query = ::evhttp_uri_get_query(requestUri);
        if (nullptr == query) {
            sendReply(req, HTTP_BADREQUEST, "Bad Request", JSON_PARAMS_REQUIRED);
            return;
        }

        const std::multimap<std::string, std::string> &params = parseQueryParameters(query);
        std::vector<std::string> hashes;

        for (std::multimap<std::string, std::string>::const_iterator it = params.find("info_hash"); it != params.end(); it++) {
            hashes.push_back(it->second);
        }

        if (hashes.size() != 1) {
            sendReply(req, HTTP_BADREQUEST, "Bad Request", JSON_INFOHASH_REQUIRED);
            return;
        }

        const std::string &info_hash = hashes.front();

        if (info_hash.length() != 40) {
            sendReply(req, HTTP_BADREQUEST, "Bad Request", JSON_INFOHASH_INVALID);
            return;
        }

        uint8_t hash [20] = {0};
        if (0 != ::str_to_hash(info_hash.c_str(), hash)) {
            sendReply(req, HTTP_BADREQUEST, "Bad Request", JSON_INFOHASH_INVALID);
            return;
        }

        if (requestMethod == EVHTTP_REQ_POST) {
            // add torrent
            if (!app->m_db.addTorrent(hash)) {
                sendReply(req, HTTP_INTERNAL, "Internal Server Error", JSON_TORRENT_ADD_FAIL);
                return;
            }
        }
        else if (requestMethod == EVHTTP_REQ_DELETE) {
            // remove torrent
            if (!app->m_db.removeTorrent(hash)) {
                sendReply(req, HTTP_INTERNAL, "Internal Server Error", JSON_TORRENT_REMOVE_FAIL);
                return;
            }
        }

        if (app->m_db.isDynamic()) {
            sendReply(req, HTTP_OK, "OK", JSON_OKAY_DYNAMIC);
        } else {
            sendReply(req, HTTP_OK, "OK", JSON_OKAY);
        }
        return;
    }

    void WebApp::viewNotFound(struct ::evhttp_request *req, void *app) {
        setCommonHeaders(req);
        sendReply(req, HTTP_NOTFOUND, "Not Found", NOT_FOUND_PAGE);
    }

    void WebApp::addHeaders(struct ::evhttp_request *req, const std::multimap<std::string, std::string>& headers) {
        struct evkeyvalq *resp_headers = ::evhttp_request_get_output_headers(req);

        for(std::multimap<std::string, std::string>::const_iterator it = headers.begin(); it != headers.end(); it++) {
            ::evhttp_add_header(resp_headers, it->first.c_str(), it->second.c_str());
        }
    }

    void WebApp::setCommonHeaders(struct ::evhttp_request *req) {
        std::multimap<std::string, std::string> headers;
        headers.insert(std::pair<std::string, std::string>("Server", "udpt"));

        addHeaders(req, headers);
    }

    std::multimap<std::string, std::string> WebApp::parseQueryParameters(const std::string& query) {
        std::string::size_type key_begin = 0, key_end = 0, value_begin = 0, value_end = 0;
        std::multimap<std::string, std::string> result;

        while (key_begin <= query.length()) {
            key_end = query.find('=', key_begin);
            if (key_end == std::string::npos) {
                // a key by itself is unacceptable...
                break;
            }

            value_begin = key_end + 1;
            value_end = query.find('&', value_begin);

            if (value_end == std::string::npos) {
                // this is the last value...
                value_end = query.length();
            }

            // insert parsed param into map:
            const std::string &key = query.substr(key_begin, key_end - key_begin);
            const std::string &value = query.substr(value_begin, value_end - value_begin);

            result.insert(std::pair<std::string, std::string>(key, value));

            // get ready for next iteration...
            key_begin = value_end + 1;

        }
        return result;
    };

    void WebApp::sendReply(struct ::evhttp_request *req, int code, const char *reason, const std::string &response) {
        sendReply(req, code, reason, response.c_str(), response.length());
    }

    void WebApp::sendReply(struct ::evhttp_request *req, int code, const char *reason, const char *response, size_t len) {
        std::shared_ptr<struct evbuffer> resp (::evbuffer_new(), ::evbuffer_free);

        if (nullptr == resp) {
            LOG_ERR("webapp", "evbuffer_new() failed to allocate buffer");
            goto error;
        }

        {
            int result = ::evbuffer_add_reference(resp.get(), response, len, nullptr, nullptr);
            if (0 != result) {
                LOG_ERR("webapp", "evbuffer_add_reference() returned " << result);
                goto error;
            }
        }

        ::evhttp_send_reply(req, code, reason, resp.get());

        // This is C++, and this is the C approach, maybe fix this in the future?
        error:
        ::evhttp_send_error(req, HTTP_INTERNAL, "Internal Server Error");
    }
}
