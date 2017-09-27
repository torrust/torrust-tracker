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

#ifndef DATABASE_HPP_
#define DATABASE_HPP_

#include <boost/program_options.hpp>

namespace UDPT
{
    namespace Data
    {
        class DatabaseException
        {
        public:
            enum EType {
                E_UNKNOWN = 0,			// Unknown error
                E_NOT_IMPLEMENTED = 1,	// not implemented
                E_CONNECTION_FAILURE = 2
            };

            DatabaseException();
            DatabaseException(EType);
            EType getErrorType();
            const char* getErrorMessage();
        private:
            EType errorNum;
        };

        class DatabaseDriver
        {
        public:
            typedef struct {
                uint8_t *info_hash;
                int32_t seeders;
                int32_t leechers;
                int32_t completed;
            } TorrentEntry;
            typedef struct {
                uint32_t ip;
                uint16_t port;
            } PeerEntry;

            enum TrackerEvents {
                EVENT_UNSPEC = 0,
                EVENT_COMPLETE = 1,
                EVENT_START = 2,
                EVENT_STOP = 3
            };

            /**
             * Opens the DB's connection
             * @param dClass Settings class ('database' class).
             */
            DatabaseDriver(const boost::program_options::variables_map& conf, bool isDynamic = false);

            /**
             * Adds a torrent to the Database. automatically done if in dynamic mode.
             * @param hash The info_hash of the torrent.
             * @return true on success. false on failure.
             */
            virtual bool addTorrent(uint8_t hash[20]);

            /**
             * Removes a torrent from the database. should be used only for non-dynamic trackers or by cleanup.
             * @param hash The info_hash to drop.
             * @return true if torrent's database was dropped or no longer exists. otherwise false (shouldn't happen - critical)
             */
            virtual bool removeTorrent(uint8_t hash[20]);

            /**
             * Checks if the Database is acting as a dynamic tracker DB.
             * @return true if dynamic. otherwise false.
             */
            bool isDynamic();

            /**
             * Checks if the torrent can be used in the tracker.
             * @param info_hash The torrent's info_hash.
             * @return true if allowed. otherwise false.
             */
            virtual bool isTorrentAllowed(uint8_t info_hash [20]);

            /**
             * Generate a Connection ID for the peer.
             * @param connectionId (Output) the generated connection ID.
             * @param ip The peer's IP (requesting peer. not remote)
             * @param port The peer's IP (remote port if tracker accepts)
             * @return
             */
            virtual bool genConnectionId(uint64_t *connectionId, uint32_t ip, uint16_t port);

            virtual bool verifyConnectionId(uint64_t connectionId, uint32_t ip, uint16_t port);

            /**
             * Updates/Adds a peer to/in the database.
             * @param peer_id the peer's peer_id
             * @param info_hash the torrent info_hash
             * @param ip IP of peer (remote ip if tracker accepts)
             * @param port TCP port of peer (remote port if tracker accepts)
             * @param downloaded total Bytes downloaded
             * @param left total bytes left
             * @param uploaded total bytes uploaded
             * @return true on success, false on failure.
             */
            virtual bool updatePeer(uint8_t peer_id [20], uint8_t info_hash [20],
                    uint32_t ip, uint16_t port,
                    int64_t downloaded, int64_t left, int64_t uploaded,
                    enum TrackerEvents event);

            /**
             * Remove a peer from a torrent (if stop action occurred, or if peer is inactive in cleanup)
             * @param peer_id The peer's peer_id
             * @param info_hash Torrent's info_hash
             * @param ip The IP of the peer (remote IP if tracker accepts)
             * @param port The TCP port (remote port if tracker accepts)
             * @return true on success. false on failure (shouldn't happen - critical)
             */
            virtual bool removePeer(uint8_t peer_id [20], uint8_t info_hash [20], uint32_t ip, uint16_t port);

            /**
             * Gets stats on a torrent
             * @param e TorrentEntry, only this info_hash has to be set
             * @return true on success, false on failure.
             */
            virtual bool getTorrentInfo(TorrentEntry *e);

            /**
             * Gets a list of peers from the database.
             * @param info_hash The torrent's info_hash
             * @param max_count The maximum amount of peers to load from the database. The amount of loaded peers is returned through this variable.
             * @param pe The list of peers. Must be pre-allocated to the size of max_count.
             * @return true on success, otherwise false (shouldn't happen).
             */
            virtual bool getPeers(uint8_t info_hash [20], int *max_count, PeerEntry *pe);

            /**
             * Cleanup the database.
             * Other actions may be locked when using this depending on the driver.
             */
            virtual void cleanup();

            /**
             * Closes the connections, and releases all other resources.
             */
            virtual ~DatabaseDriver();

        protected:
            const boost::program_options::variables_map& m_conf;
        private:
            bool is_dynamic;
        };
    };
};


#endif /* DATABASE_HPP_ */
