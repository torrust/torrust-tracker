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

#include "database.hpp"

namespace UDPT
{
    namespace Data
    {
        DatabaseDriver::DatabaseDriver(const boost::program_options::variables_map& conf, bool isDynamic) : m_conf(conf)
        {
            this->is_dynamic = isDynamic;
        }

        bool DatabaseDriver::addTorrent(uint8_t hash [20])
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::removeTorrent(uint8_t hash[20])
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::isDynamic()
        {
            return this->is_dynamic;
        }

        bool DatabaseDriver::genConnectionId(uint64_t *cid, uint32_t ip, uint16_t port)
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::verifyConnectionId(uint64_t cid, uint32_t ip, uint16_t port)
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::updatePeer(uint8_t peer_id [20], uint8_t info_hash [20],
                    uint32_t ip, uint16_t port,
                    int64_t downloaded, int64_t left, int64_t uploaded,
                    enum TrackerEvents event)
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::removePeer (uint8_t peer_id [20], uint8_t info_hash [20], uint32_t ip, uint16_t port)
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::getTorrentInfo (TorrentEntry *e)
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::getPeers (uint8_t info_hash [20], int *max_count, PeerEntry *pe)
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        void DatabaseDriver::cleanup()
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        bool DatabaseDriver::isTorrentAllowed(uint8_t info_hash[20])
        {
            throw DatabaseException (DatabaseException::E_NOT_IMPLEMENTED);
        }

        DatabaseDriver::~DatabaseDriver()
        {
        }

        /*-- Exceptions --*/
        static const char *EMessages[] = {
                "Unknown Error",
                "Not Implemented",
                "Failed to connect to database"
        };

        DatabaseException::DatabaseException()
        {
            this->errorNum = E_UNKNOWN;
        }

        DatabaseException::DatabaseException(enum EType e)
        {
            this->errorNum = e;
        }

        enum DatabaseException::EType DatabaseException::getErrorType()
        {
            return this->errorNum;
        }

        const char* DatabaseException::getErrorMessage()
        {
            return EMessages[this->errorNum];
        }
    };
};
