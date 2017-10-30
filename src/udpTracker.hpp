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

#include <stdint.h>
#include <chrono>
#include <algorithm>
#include <string>
#include <sstream>
#include <list>
#include <ctime>
#include <atomic>
#include <thread>
#include <mutex>
#include <condition_variable>

#include <boost/program_options.hpp>

#ifdef WIN32
#else
#include <sys/socket.h>
#include <netinet/in.h>
#endif

#include "exceptions.h"
#include "db/database.hpp"

#define UDPT_DYNAMIC			(0x01)	// Track Any info_hash?
#define UDPT_ALLOW_REMOTE_IP	(0x02)	// Allow client's to send other IPs?
#define UDPT_ALLOW_IANA_IP		(0x04)	// allow IP's like 127.0.0.1 or other IANA reserved IPs?
#define UDPT_VALIDATE_CLIENT	(0x08)	// validate client before adding to Database? (check if connection is open?)


namespace UDPT
{
    class UDPTracker
    {
    public:
        typedef struct udp_connection_request
        {
            uint64_t connection_id;
            uint32_t action;
            uint32_t transaction_id;
        } ConnectionRequest;

        typedef struct udp_connection_response
        {
            uint32_t action;
            uint32_t transaction_id;
            uint64_t connection_id;
        } ConnectionResponse;

        typedef struct udp_announce_request
        {
            uint64_t connection_id;
            uint32_t action;
            uint32_t transaction_id;
            uint8_t info_hash [20];
            uint8_t peer_id [20];
            uint64_t downloaded;
            uint64_t left;
            uint64_t uploaded;
            uint32_t event;
            uint32_t ip_address;
            uint32_t key;
            int32_t num_want;
            uint16_t port;
        } AnnounceRequest;

        typedef struct udp_announce_response
        {
            uint32_t action;
            uint32_t transaction_id;
            uint32_t interval;
            uint32_t leechers;
            uint32_t seeders;

            uint8_t *peer_list_data;
        } AnnounceResponse;

        typedef struct udp_scrape_request
        {
            uint64_t connection_id;
            uint32_t action;
            uint32_t transaction_id;

            uint8_t *torrent_list_data;
        } ScrapeRequest;

        typedef struct udp_scrape_response
        {
            uint32_t action;
            uint32_t transaction_id;

            uint8_t *data;
        } ScrapeResponse;

        typedef struct udp_error_response
        {
            uint32_t action;
            uint32_t transaction_id;
            char *message;
        } ErrorResponse;

        enum StartStatus
        {
            START_OK = 0,
            START_ESOCKET_FAILED = 1,
            START_EBIND_FAILED = 2
        };

        /**
         * Initializes the UDP Tracker.
         * @param settings Settings to start server with
         */
        UDPTracker(const boost::program_options::variables_map& conf);

        /**
         * Starts the Initialized instance.
         */
        void start();

        /**
         * Terminates tracker.
         */
        void stop();

        /**
         * Joins worker threads
         */
        void wait();

        /**
         * Destroys resources that were created by constructor
         * @param usi Instance to destroy.
         */
        virtual ~UDPTracker();

        std::shared_ptr<UDPT::Data::DatabaseDriver> m_conn;

    private:
        int m_sock;
        struct sockaddr_in m_localEndpoint;
        uint16_t m_port;
        uint8_t m_threadCount;
        bool m_isDynamic;
        bool m_allowRemotes;
        bool m_allowIANA_IPs;
        std::vector<std::thread> m_threads;
        uint32_t m_announceInterval;
        uint32_t m_cleanupInterval;
        std::atomic_bool m_shouldRun;

        std::mutex m_maintenanceMutex;
        std::condition_variable m_maintenanceCondition;

        const boost::program_options::variables_map& m_conf;

        static void _thread_start(UDPTracker *usi);
        static void _maintainance_start(UDPTracker* usi);

        static int resolveRequest(UDPTracker *usi, struct sockaddr_in *remote, char *data, int r);

        static int handleConnection(UDPTracker *usi, struct sockaddr_in *remote, char *data);
        static int handleAnnounce(UDPTracker *usi, struct sockaddr_in *remote, char *data);
        static int handleScrape(UDPTracker *usi, struct sockaddr_in *remote, char *data, int len);

        static int sendError(UDPTracker *, struct sockaddr_in *remote, uint32_t transId, const std::string &);

        static int isIANAIP(uint32_t ip);
    };
};
