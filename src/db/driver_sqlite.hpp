/*
 *	Copyright © 2012-2016 Naim A.
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

#ifndef DATABASE_H_
#define DATABASE_H_

#include <stdint.h>
#include "database.hpp"
#include <sqlite3.h>

namespace UDPT
{
	namespace Data
	{
		class SQLite3Driver : public DatabaseDriver
		{
		public:
			SQLite3Driver(const boost::program_options::variables_map& conf, bool isDyn = false);
			bool addTorrent(uint8_t info_hash[20]);
			bool removeTorrent(uint8_t info_hash[20]);
			bool genConnectionId(uint64_t *connId, uint32_t ip, uint16_t port);
			bool verifyConnectionId(uint64_t connId, uint32_t ip, uint16_t port);
			bool updatePeer(uint8_t peer_id [20], uint8_t info_hash [20], uint32_t ip, uint16_t port, int64_t downloaded, int64_t left, int64_t uploaded, enum TrackerEvents event);
			bool removePeer(uint8_t peer_id [20], uint8_t info_hash [20], uint32_t ip, uint16_t port);
			bool getTorrentInfo(TorrentEntry *e);
			bool isTorrentAllowed(uint8_t info_hash[20]);
			bool getPeers(uint8_t info_hash [20], int *max_count, PeerEntry *pe);
			void cleanup();

			virtual ~SQLite3Driver();
		private:
			sqlite3 *db;
			boost::log::sources::severity_channel_logger_mt<> m_logger;

			void doSetup();
		};
	};
};

#endif /* DATABASE_H_ */
