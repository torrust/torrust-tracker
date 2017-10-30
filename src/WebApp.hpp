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

#pragma once
#include <event2/http.h>
#include <thread>
#include <condition_variable>

#include "db/database.hpp"

namespace UDPT
{
    class WebApp {
    public:
        WebApp(UDPT::Data::DatabaseDriver& db, const std::string& listenIP, uint16_t listenPort);

        virtual ~WebApp();

        void start();

        void stop();

    private:
        static const std::string ANNOUNCE_PAGE;
        static const std::string NOT_FOUND_PAGE;
        static const std::string HOME_PAGE;
        static const std::string JSON_INVALID_METHOD;
        static const std::string JSON_INTERNAL_ERROR;
        static const std::string JSON_PARAMS_REQUIRED;
        static const std::string JSON_INFOHASH_REQUIRED;
        static const std::string JSON_INFOHASH_INVALID;
        static const std::string JSON_TORRENT_ADD_FAIL;
        static const std::string JSON_TORRENT_REMOVE_FAIL;
        static const std::string JSON_OKAY;
        static const std::string JSON_OKAY_DYNAMIC;

        UDPT::Data::DatabaseDriver& m_db;

        const std::string m_listenIP;
        uint16_t m_listenPort;

        std::thread m_workerThread;
        std::atomic_bool m_isRunning;

        // Be Aware: The order of these members are important
        // we wouldn't want to free event_base before http_server...
        std::shared_ptr<struct event_base> m_eventBase;
        std::shared_ptr<struct evhttp> m_httpServer;

        static void workerThread(WebApp *);

        static void viewApiTorrents(struct ::evhttp_request *, void *);

        static void viewNotFound(struct ::evhttp_request *, void *);

        static void addHeaders(struct ::evhttp_request *, const std::multimap<std::string, std::string>& headers);

        static void setCommonHeaders(struct ::evhttp_request *);

        static std::multimap<std::string, std::string> parseQueryParameters(const std::string& query);

        // these methods are currently only safe for static strings...
        static void sendReply(struct ::evhttp_request *req, int code, const char *reason, const std::string &response);
        static void sendReply(struct ::evhttp_request *req, int code, const char *reason, const char *response, size_t len);
    };
}