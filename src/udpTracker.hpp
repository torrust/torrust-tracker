/*
 *	Copyright © 2012,2013 Naim A.
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

#ifndef UDPTRACKER_H_
#define UDPTRACKER_H_


#include <stdint.h>
#include "multiplatform.h"
#include "db/driver_sqlite.hpp"
#include "settings.hpp"

#include <string>
using namespace std;

#define UDPT_DYNAMIC			0x01	// Track Any info_hash?
#define UDPT_ALLOW_REMOTE_IP	0x02	// Allow client's to send other IPs?
#define UDPT_ALLOW_IANA_IP		0x04	// allow IP's like 127.0.0.1 or other IANA reserved IPs?
#define UDPT_VALIDATE_CLIENT	0x08	// validate client before adding to Database? (check if connection is open?)


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
		UDPTracker (Settings *);

		/**
		 * Starts the Initialized instance.
		 * @return 0 on success, otherwise non-zero.
		 */
		enum StartStatus start ();

		/**
		 * Destroys resources that were created by constructor
		 * @param usi Instance to destroy.
		 */
		virtual ~UDPTracker ();

		Data::DatabaseDriver *conn;
	private:
		SOCKET sock;
		uint16_t port;
		uint8_t thread_count;
		bool isRunning;
		HANDLE *threads;
		uint32_t announce_interval;
		uint32_t cleanup_interval;

		uint8_t settings;
		Settings *o_settings;

#ifdef WIN32
		static DWORD _thread_start (LPVOID arg);
		static DWORD _maintainance_start (LPVOID arg);
#elif defined (linux)
		static void* _thread_start (void *arg);
		static void* _maintainance_start (void *arg);
#endif

		static int resolveRequest (UDPTracker *usi, SOCKADDR_IN *remote, char *data, int r);

		static int handleConnection (UDPTracker *usi, SOCKADDR_IN *remote, char *data);
		static int handleAnnounce (UDPTracker *usi, SOCKADDR_IN *remote, char *data);
		static int handleScrape (UDPTracker *usi, SOCKADDR_IN *remote, char *data, int len);

		static int sendError (UDPTracker *, SOCKADDR_IN *remote, uint32_t transId, const string &);

	};
};

#endif /* UDPTRACKER_H_ */
