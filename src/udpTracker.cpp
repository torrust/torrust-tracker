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

#include "udpTracker.hpp"
#include "logging.hpp"
#include "tools.h"
#include "db/driver_sqlite.hpp"

#ifdef WIN32

#elif defined(__linux__) || defined(__FreeBSD__)
#include <arpa/inet.h>
#endif

using namespace UDPT::Data;

#define UDP_BUFFER_SIZE		2048

namespace UDPT
{
    UDPTracker::UDPTracker(const boost::program_options::variables_map& conf) : m_conf(conf) {
        this->m_allowRemotes = conf["tracker.allow_remotes"].as<bool>();
        this->m_allowIANA_IPs = conf["tracker.allow_iana_ips"].as<bool>();
        this->m_isDynamic = conf["tracker.is_dynamic"].as<bool>();

        this->m_announceInterval = conf["tracker.announce_interval"].as<unsigned>();
        this->m_cleanupInterval = conf["tracker.cleanup_interval"].as<unsigned>();
        this->m_port = conf["tracker.port"].as<unsigned short>();
        this->m_threadCount = conf["tracker.threads"].as<unsigned>() + 1;

        this->m_localEndpoint.sin_family = AF_INET;
        this->m_localEndpoint.sin_port = ::m_hton16(m_port);
        this->m_localEndpoint.sin_addr.s_addr = 0L;

        this->m_conn = std::shared_ptr<DatabaseDriver>(new Data::SQLite3Driver(m_conf, this->m_isDynamic));
    }

    UDPTracker::~UDPTracker() {
    }

    void UDPTracker::start() {
        int sock;
        int r,		// saves results
            i,		// loop index
            yup;	// just to set TRUE
        std::string dbname;// saves the Database name.

        sock = ::socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if (sock == -1)
        {
            LOG_FATAL("udp-tracker", "Failed to create socket. error=" << errno);
            throw UDPT::UDPTException("Failed to create socket");
        }

        yup = 1;
        ::setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, (const char*)&yup, 1);

        {
            // don't block recvfrom for too long.
#if defined(__linux__) || defined(__FreeBSD__)
            timeval timeout = { 0 };
            timeout.tv_sec = 5;
#elif defined(WIN32)
            DWORD timeout = 5000;
#endif
            ::setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, reinterpret_cast<const char*>(&timeout), sizeof(timeout));
        }

        this->m_localEndpoint.sin_family = AF_INET;
        r = ::bind(sock, reinterpret_cast<struct sockaddr*>(&this->m_localEndpoint), sizeof(struct sockaddr_in));

        if (r == -1)
        {
            LOG_FATAL("udp-tracker", "Failed to bind socket. error=" << errno);

#ifdef WIN32
            ::closesocket(sock);
#elif defined(__linux__) || defined(__FreeBSD__)
            ::close(sock);
#endif
            throw UDPT::UDPTException("Failed to bind socket.");
        }

        this->m_sock = sock;

        LOG_INFO("udp-tracker", "Tracker bound to " << ::inet_ntoa(this->m_localEndpoint.sin_addr));

        // create maintainer thread.
        m_threads.push_back(std::thread(UDPTracker::_maintainance_start, this));
        LOG_INFO("udp-tracker", "Started maintenance thread @" << m_threads.back().get_id());

        m_shouldRun = true;
        for (i = 1;i < this->m_threadCount; i++)
        {
            m_threads.push_back(std::thread(UDPTracker::_thread_start, this));
            LOG_INFO("udp-tracker", "Started worker thread @" << m_threads.back().get_id());
        }
    }

    void UDPTracker::stop() {
        // tell workers to stop running...
        m_shouldRun = false;

        // stop maintenance thread's sleep...
        m_maintenanceCondition.notify_one();
    }

    /**
     * @brief blocks until all threads die.
     * @note This method should be called only once, preferably by the main thread.
     * **/
    void UDPTracker::wait() {
        for (std::vector<std::thread>::iterator it = m_threads.begin(); it != m_threads.end(); ++it)
        {
            it->join();
        }

        LOG_INFO("udp-tracker", "UDP Tracker terminated");

#if defined(__linux__) || defined(__FreeBSD__)
        ::close(m_sock);
#elif defined(WIN32)
        ::closesocket(m_sock);
#endif
    }

    int UDPTracker::sendError(UDPTracker* usi, struct sockaddr_in* remote, uint32_t transactionID, const std::string &msg) {
        struct udp_error_response error;
        int msg_sz,	// message size to send.
            i;		// copy loop
        char buff [1024];	// more than reasonable message size...

        error.action = m_hton32 (3);
        error.transaction_id = transactionID;
        error.message = (char*)msg.c_str();

        msg_sz = 4 + 4 + 1 + msg.length();

        // test against overflow message. resolves issue 4.
        if (msg_sz > 1024)
            return -1;

        ::memcpy(buff, &error, 8);
        for (i = 8;i <= msg_sz;i++)
        {
            buff[i] = msg[i - 8];
        }

        ::sendto(usi->m_sock, buff, msg_sz, 0, reinterpret_cast<struct sockaddr*>(remote), sizeof(*remote));

        LOG_DEBUG("udp-tracker", "Error sent to " << ::inet_ntoa(remote->sin_addr) << ", '" << msg << "' (len=" << msg_sz << ")");

        return 0;
    }

    int UDPTracker::handleConnection(UDPTracker *usi, struct sockaddr_in *remote, char *data) {
        ConnectionRequest *req = reinterpret_cast<ConnectionRequest*>(data);
        ConnectionResponse resp;

        resp.action = m_hton32(0);
        resp.transaction_id = req->transaction_id;

        if (!usi->m_conn->genConnectionId(&resp.connection_id,
                m_hton32(remote->sin_addr.s_addr),
                m_hton16(remote->sin_port)))
        {
            return 1;
        }

        ::sendto(usi->m_sock, (char*)&resp, sizeof(ConnectionResponse), 0, reinterpret_cast<struct sockaddr*>(remote), sizeof(struct sockaddr_in));

        return 0;
    }

    int UDPTracker::handleAnnounce(UDPTracker *usi, struct sockaddr_in *remote, char *data) {
        AnnounceRequest *req;
        AnnounceResponse *resp;
        int q,		// peer counts
            bSize,	// message size
            i;		// loop index
        DatabaseDriver::TorrentEntry tE;

        uint8_t buff[1028];	// Reasonable buffer size. (header+168 peers)

        req = (AnnounceRequest*)data;

        if (!usi->m_conn->verifyConnectionId(req->connection_id,
            m_hton32(remote->sin_addr.s_addr),
            m_hton16(remote->sin_port)))
        {
            return 1;
        }

        // change byte order:
        req->port = m_hton16(req->port);
        req->ip_address = m_hton32(req->ip_address);
        req->downloaded = m_hton64(req->downloaded);
        req->event = m_hton32(req->event);	// doesn't really matter for this tracker
        req->uploaded = m_hton64(req->uploaded);
        req->num_want = m_hton32(req->num_want);
        req->left = m_hton64(req->left);

        if (!usi->m_allowRemotes && req->ip_address != 0)
        {
            UDPTracker::sendError(usi, remote, req->transaction_id, "Tracker doesn't allow remote IP's; Request ignored.");
            return 0;
        }

        if (!usi->m_conn->isTorrentAllowed(req->info_hash))
        {
            UDPTracker::sendError(usi, remote, req->transaction_id, "info_hash not registered.");
            return 0;
        }

        // load peers
        q = 30;
        if (req->num_want >= 1)
            q = std::min<int>(q, req->num_want);

        DatabaseDriver::TrackerEvents event;

        {
            std::shared_ptr<DatabaseDriver::PeerEntry> peersSptr = std::shared_ptr<DatabaseDriver::PeerEntry>(new DatabaseDriver::PeerEntry[q]);
            DatabaseDriver::PeerEntry *peers = peersSptr.get();

            switch (req->event)
            {
            case 1:
                event = DatabaseDriver::EVENT_COMPLETE;
                break;
            case 2:
                event = DatabaseDriver::EVENT_START;
                break;
            case 3:
                event = DatabaseDriver::EVENT_STOP;
                break;
            default:
                event = DatabaseDriver::EVENT_UNSPEC;
                break;
            }

            if (event == DatabaseDriver::EVENT_STOP)
                q = 0;	// no need for peers when stopping.

            if (q > 0)
                usi->m_conn->getPeers(req->info_hash, &q, peers);

            bSize = 20; // header is 20 bytes
            bSize += (6 * q); // + 6 bytes per peer.

            tE.info_hash = req->info_hash;
            usi->m_conn->getTorrentInfo(&tE);

            resp = (AnnounceResponse*)buff;
            resp->action = m_hton32(1);
            resp->interval = m_hton32(usi->m_announceInterval);
            resp->leechers = m_hton32(tE.leechers);
            resp->seeders = m_hton32(tE.seeders);
            resp->transaction_id = req->transaction_id;

            for (i = 0; i < q; i++)
            {
                int x = i * 6;
                // network byte order!!!

                // IP
                buff[20 + x] = ((peers[i].ip & (0xff << 24)) >> 24);
                buff[21 + x] = ((peers[i].ip & (0xff << 16)) >> 16);
                buff[22 + x] = ((peers[i].ip & (0xff << 8)) >> 8);
                buff[23 + x] = (peers[i].ip & 0xff);

                // port
                buff[24 + x] = ((peers[i].port & (0xff << 8)) >> 8);
                buff[25 + x] = (peers[i].port & 0xff);

            }
        }
        ::sendto(usi->m_sock, (char*)buff, bSize, 0, reinterpret_cast<struct sockaddr*>(remote), sizeof(struct sockaddr_in));
        LOG_DEBUG("udp-tracker", "Announce request from " << ::inet_ntoa(remote->sin_addr) << " (event=" << event << "), Sent " << q << " peers");

        // update DB.
        uint32_t ip;
        if (req->ip_address == 0) // default
            ip = m_hton32 (remote->sin_addr.s_addr);
        else
            ip = req->ip_address;
        usi->m_conn->updatePeer(req->peer_id, req->info_hash, ip, req->port,
                req->downloaded, req->left, req->uploaded, event);

        return 0;
    }

    int UDPTracker::handleScrape(UDPTracker *usi, struct sockaddr_in *remote, char *data, int len) {
        ScrapeRequest *sR = reinterpret_cast<ScrapeRequest*>(data);
        int v,	// validation helper
            c,	// torrent counter
            i,	// loop counter
            j;	// loop counter
        uint8_t hash [20];
        ScrapeResponse *resp;
        uint8_t buffer [1024];	// up to 74 torrents can be scraped at once (17*74+8) < 1024

        // validate request length:
        v = len - 16;
        if (v < 0 || v % 20 != 0)
        {
            UDPTracker::sendError(usi, remote, sR->transaction_id, "Bad scrape request.");
            return 0;
        }

        if (!usi->m_conn->verifyConnectionId(sR->connection_id,
                m_hton32(remote->sin_addr.s_addr),
                m_hton16(remote->sin_port)))
        {
            LOG_DEBUG("udp-tracker", "Bad connection id from " << ::inet_ntoa(remote->sin_addr));
            return 1;
        }

        // get torrent count.
        c = v / 20;

        resp = reinterpret_cast<ScrapeResponse*>(buffer);
        resp->action = m_hton32(2);
        resp->transaction_id = sR->transaction_id;

        for (i = 0;i < c;i++)
        {
            int32_t *seeders,
                *completed,
                *leechers;

            for (j = 0; j < 20;j++)
                hash[j] = data[j + (i*20)+16];

            seeders = (int32_t*)&buffer[i*12+8];
            completed = (int32_t*)&buffer[i*12+12];
            leechers = (int32_t*)&buffer[i*12+16];

            DatabaseDriver::TorrentEntry tE;
            tE.info_hash = hash;
            if (!usi->m_conn->getTorrentInfo(&tE))
            {
                sendError(usi, remote, sR->transaction_id, "Scrape Failed: couldn't retrieve torrent data");
                return 0;
            }

            *seeders = m_hton32(tE.seeders);
            *completed = m_hton32(tE.completed);
            *leechers = m_hton32(tE.leechers);
        }

        ::sendto(usi->m_sock, reinterpret_cast<const char*>(buffer), sizeof(buffer), 0, reinterpret_cast<struct sockaddr*>(remote), sizeof(struct sockaddr_in));
        LOG_DEBUG("udp-tracker", "Scrape request from " << ::inet_ntoa(remote->sin_addr) << ", Sent " << c << " torrents");

        return 0;
    }

    int UDPTracker::isIANAIP(uint32_t ip) {
        uint8_t x = (ip % 256);
        if (x == 0 || x == 10 || x == 127 || x >= 224)
            return 1;
        return 0;
    }

    int UDPTracker::resolveRequest(UDPTracker *usi, struct sockaddr_in *remote, char *data, int r) {
        ConnectionRequest* cR = reinterpret_cast<ConnectionRequest*>(data);
        uint32_t action;

        action = m_hton32(cR->action);

        if (!usi->m_allowIANA_IPs)
        {
            if (isIANAIP(remote->sin_addr.s_addr))
            {
                LOG_DEBUG("udp-tracker", "Request from IANA reserved IP rejected (" << ::inet_ntoa(remote->sin_addr) << ")");
                return 0;	// Access Denied: IANA reserved IP.
            }
        }

        if (action == 0 && r >= 16)
            return UDPTracker::handleConnection(usi, remote, data);
        else if (action == 1 && r >= 98)
            return UDPTracker::handleAnnounce(usi, remote, data);
        else if (action == 2)
            return UDPTracker::handleScrape(usi, remote, data, r);
        else
        {
            UDPTracker::sendError(usi, remote, cR->transaction_id, "Tracker couldn't understand Client's request.");
            return -1;
        }
    }

    void UDPTracker::_thread_start(UDPTracker *usi) {
        struct sockaddr_in remoteAddr;
        char tmpBuff[UDP_BUFFER_SIZE];

        unsigned int addrSz;

        addrSz = sizeof(struct sockaddr_in);

        while (usi->m_shouldRun)
        {
            // peek into the first 12 bytes of data; determine if connection request or announce request.
            int r = ::recvfrom(usi->m_sock, (char*)tmpBuff, UDP_BUFFER_SIZE, 0, reinterpret_cast<struct sockaddr*>(&remoteAddr), &addrSz);
            if (r <= 0)
            {
                std::this_thread::yield();
                continue;
            }

            UDPTracker::resolveRequest(usi, &remoteAddr, tmpBuff, r);
        }

        LOG_INFO("udp-tracker", "worker " << std::this_thread::get_id() << " exited.");
    }

    void UDPTracker::_maintainance_start(UDPTracker* usi) {
        std::unique_lock<std::mutex> lk (usi->m_maintenanceMutex);

        while (true)
        {
            if (std::cv_status::no_timeout == usi->m_maintenanceCondition.wait_for(lk, std::chrono::seconds(usi->m_cleanupInterval))) {
                break;
            }

            LOG_INFO("udp-tracker", "Maintenance running...");
            usi->m_conn->cleanup();
        }

        lk.unlock();
        LOG_INFO("udp-tracker", "Maintenance thread " << std::this_thread::get_id() << " exited.");
    }
};
