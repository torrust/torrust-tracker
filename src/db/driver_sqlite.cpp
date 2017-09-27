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

#include "driver_sqlite.hpp"
#include "../tools.h"
#include <ctime>
#include <sstream>
#include <fstream>
#include <iostream>
#include <cassert>
#include <cstring> // memcpy
#include "../multiplatform.h"
#include "../logging.hpp"

using namespace std;

namespace UDPT
{
    namespace Data
    {
        static const char hexadecimal[] = "0123456789abcdef";

        static char* _to_hex_str (const uint8_t *hash, char *data)
        {
            int i;
            for (i = 0;i < 20;i++)
            {
                data[i * 2] = hexadecimal[hash[i] / 16];
                data[i * 2 + 1] = hexadecimal[hash[i] % 16];
            }
            data[40] = '\0';
            return data;
        }

        static uint8_t* _hash_to_bin (const char *hash, uint8_t *data)
        {
            for (int i = 0;i < 20;i++)
            {
                data [i] = 0;
                char a = hash[i * 2];
                char b = hash[i * 2 + 1];

                assert ( (a >= 'a' && a <= 'f') || (a >= '0' && a <= '9') );
                assert ( (b >= 'a' && b <= 'f') || (b >= '0' && b <= '9') );

                data[i] = ( (a >= '0' && a <= 'f') ? (a - '0') : (a - 'f' + 10) );
                data[i] <<= 4;
                data[i] = ( (b >= '0' && b <= 'f') ? (b - '0') : (b - 'f' + 10) );
            }

            return data;
        }

        SQLite3Driver::SQLite3Driver(const boost::program_options::variables_map& conf, bool isDyn) : DatabaseDriver(conf, isDyn)
        {
            int r;
            bool doSetup;

            fstream fCheck;
            string filename = m_conf["db.param"].as<std::string>();

            fCheck.open(filename.c_str(), ios::binary | ios::in);
            if (fCheck.is_open())
            {
                doSetup = false;
                fCheck.close();
            }
            else
                doSetup = true;

            r = sqlite3_open(filename.c_str(), &this->db);
            if (r != SQLITE_OK)
            {
                LOG_FATAL("db-sqlite", "Failed to connect DB. sqlite returned " << r);
                sqlite3_close(this->db);
                throw DatabaseException (DatabaseException::E_CONNECTION_FAILURE);
            }

            if (doSetup)
                this->doSetup();
        }

        void SQLite3Driver::doSetup()
        {
            char *eMsg = NULL;
            LOG_INFO("db-sqlite", "Setting up database...");
            // for quicker stats.
            sqlite3_exec(this->db, "CREATE TABLE stats ("
                    "info_hash blob(20) UNIQUE,"
                    "completed INTEGER DEFAULT 0,"
                    "leechers INTEGER DEFAULT 0,"
                    "seeders INTEGER DEFAULT 0,"
                    "last_mod INTEGER DEFAULT 0"
                    ")", NULL, NULL, &eMsg);

            sqlite3_exec(this->db, "CREATE TABLE torrents ("
                    "info_hash blob(20) UNIQUE,"
                    "created INTEGER"
                    ")", NULL, NULL, &eMsg);
        }

        bool SQLite3Driver::getTorrentInfo(TorrentEntry *e)
        {
            bool gotInfo = false;

            const char sql[] = "SELECT seeders,leechers,completed FROM 'stats' WHERE info_hash=?";
            sqlite3_stmt *stmt;

            e->seeders = 0;
            e->leechers = 0;
            e->completed = 0;


            sqlite3_prepare (this->db, sql, -1, &stmt, NULL);
            sqlite3_bind_blob (stmt, 1, (void*)e->info_hash, 20, NULL);

            if (sqlite3_step(stmt) == SQLITE_ROW)
            {
                e->seeders = sqlite3_column_int (stmt, 0);
                e->leechers = sqlite3_column_int (stmt, 1);
                e->completed = sqlite3_column_int (stmt, 2);

                gotInfo = true;
            }

            sqlite3_finalize (stmt);

            return gotInfo;
        }

        bool SQLite3Driver::getPeers (uint8_t info_hash [20], int *max_count, PeerEntry *pe)
        {
            string sql;
            char hash [50];
            sqlite3_stmt *stmt;
            int r, i;

            to_hex_str(info_hash, hash);

            sql = "SELECT ip,port FROM 't";
            sql += hash;
            sql += "' LIMIT ?";

            sqlite3_prepare(this->db, sql.c_str(), sql.length(), &stmt, NULL);
            sqlite3_bind_int(stmt, 1, *max_count);

            i = 0;
            while (*max_count > i)
            {
                r = sqlite3_step(stmt);
                if (r == SQLITE_ROW)
                {
                    const char *ip = (const char*)sqlite3_column_blob (stmt, 0);
                    const char *port = (const char*)sqlite3_column_blob (stmt, 1);

                    memcpy(&pe[i].ip, ip, 4);
                    memcpy(&pe[i].port, port, 2);

                    i++;
                }
                else
                {
                    break;
                }
            }

            sqlite3_finalize(stmt);

            *max_count = i;

            return true;
        }

        bool SQLite3Driver::updatePeer(uint8_t peer_id[20], uint8_t info_hash[20], uint32_t ip, uint16_t port, int64_t downloaded, int64_t left, int64_t uploaded, enum TrackerEvents event)
        {
            char xHash [50]; // we just need 40 + \0 = 41.
            sqlite3_stmt *stmt;
            string sql;
            int r;

            char *hash = xHash;
            to_hex_str(info_hash, hash);

            addTorrent (info_hash);


            sql = "REPLACE INTO 't";
            sql += hash;
            sql += "' (peer_id,ip,port,uploaded,downloaded,left,last_seen) VALUES (?,?,?,?,?,?,?)";

            sqlite3_prepare(this->db, sql.c_str(), sql.length(), &stmt, NULL);

            sqlite3_bind_blob(stmt, 1, (void*)peer_id, 20, NULL);
            sqlite3_bind_blob(stmt, 2, (void*)&ip, 4, NULL);
            sqlite3_bind_blob(stmt, 3, (void*)&port, 2, NULL);
            sqlite3_bind_blob(stmt, 4, (void*)&uploaded, 8, NULL);
            sqlite3_bind_blob(stmt, 5, (void*)&downloaded, 8, NULL);
            sqlite3_bind_blob(stmt, 6, (void*)&left, 8, NULL);
            sqlite3_bind_int(stmt, 7, time(NULL));

            r = sqlite3_step(stmt);
            sqlite3_finalize(stmt);

            sql = "REPLACE INTO stats (info_hash,last_mod) VALUES (?,?)";
            sqlite3_prepare (this->db, sql.c_str(), sql.length(), &stmt, NULL);
            sqlite3_bind_blob (stmt, 1, hash, 20, NULL);
            sqlite3_bind_int (stmt, 2, time(NULL));
            sqlite3_step (stmt);
            sqlite3_finalize (stmt);

            return r;
        }

        bool SQLite3Driver::addTorrent (uint8_t info_hash[20])
        {
            char xHash [41];
            char *err_msg;
            int r;

            _to_hex_str(info_hash, xHash);

            sqlite3_stmt *stmt;
            sqlite3_prepare(this->db, "INSERT INTO torrents (info_hash,created) VALUES (?,?)", -1, &stmt, NULL);
            sqlite3_bind_blob(stmt, 1, info_hash, 20, NULL);
            sqlite3_bind_int(stmt, 2, time(NULL));
            sqlite3_step(stmt);
            sqlite3_finalize(stmt);

            string sql = "CREATE TABLE IF NOT EXISTS 't";
            sql += xHash;
            sql += "' (";
            sql += "peer_id blob(20),"
                    "ip blob(4),"
                    "port blob(2),"
                    "uploaded blob(8)," // uint64
                    "downloaded blob(8),"
                    "left blob(8),"
                    "last_seen INT DEFAULT 0";

            sql += ", CONSTRAINT c1 UNIQUE (ip,port) ON CONFLICT REPLACE)";

            // create table.
            r = sqlite3_exec(this->db, sql.c_str(), NULL, NULL, &err_msg);

            if (SQLITE_OK == r)
            {
                return true;
            }
            else
            {
                return false;
            }
        }

        bool SQLite3Driver::isTorrentAllowed(uint8_t *info_hash)
        {
            if (this->isDynamic())
                return true;
            sqlite3_stmt *stmt;
            sqlite3_prepare(this->db, "SELECT COUNT(*) FROM torrents WHERE info_hash=?", -1, &stmt, NULL);
            sqlite3_bind_blob(stmt, 1, info_hash, 20, NULL);
            sqlite3_step(stmt);

            int n = sqlite3_column_int(stmt, 0);
            sqlite3_finalize(stmt);

            return (n == 1);
        }

        void SQLite3Driver::cleanup()
        {
            LOG_INFO("db-sqlite", "Cleaning up...");
            int exp = time (NULL) - 7200;	// 2 hours,  expired.
            int r = 0;

            // drop all peers with no activity for 2 hours.
            sqlite3_stmt *getTables;
            // torrent table names: t<hex-of-sha-1>
            r = sqlite3_prepare(this->db, "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 't________________________________________'", -1, &getTables, NULL);
            if (r != SQLITE_OK) {
                LOG_ERR("db-sqlite", "Failed fetch tables from DB for cleanup.");
                return;
            }

            uint8_t buff [20];
            sqlite3_stmt *updateStats;
            r = sqlite3_prepare(this->db, "REPLACE INTO stats (info_hash,seeders,leechers,last_mod) VALUES (?,?,?,?)", -1, &updateStats, NULL);
            if (r != SQLITE_OK) {
                LOG_ERR("db-sqlite", "Failed to prepare update stats query.");
                return;
            }

            while (sqlite3_step(getTables) == SQLITE_ROW)
            {
                char* tblN = (char*)sqlite3_column_text(getTables, 0);
                stringstream sStr;
                sStr << "DELETE FROM " << tblN << " WHERE last_seen<" << exp;

                r = sqlite3_exec(this->db, sStr.str().c_str(), NULL, NULL, NULL);
                if (r != SQLITE_OK) {
                    LOG_ERR("db-sqlite", "Failed to execute cleanup for table '" << tblN << "'.");
                    continue;
                }

                sStr.str (string());
                sStr << "SELECT left,COUNT(*) FROM " << tblN << " GROUP BY left==0";

                sqlite3_stmt *collectStats;

                r = sqlite3_prepare(this->db, sStr.str().c_str(), sStr.str().length(), &collectStats, NULL);

                if (r != SQLITE_OK)
                {
                    LOG_ERR("db-sqlite", "Failed while trying to prepare stats query for '" << tblN << "', sqlite returned " << r);
                    continue;
                }

                int seeders = 0, leechers = 0;
                while (sqlite3_step(collectStats) == SQLITE_ROW) // expecting two results.
                {
                    if (sqlite3_column_int(collectStats, 0) == 0)
                        seeders = sqlite3_column_int (collectStats, 1);
                    else
                        leechers = sqlite3_column_int (collectStats, 1);
                }
                sqlite3_finalize(collectStats);

                sqlite3_bind_blob(updateStats, 1, _hash_to_bin((const char*)(tblN + 1), buff), 20, NULL);
                sqlite3_bind_int(updateStats, 2, seeders);
                sqlite3_bind_int(updateStats, 3, leechers);
                sqlite3_bind_int(updateStats, 4, time (NULL));

                sqlite3_step(updateStats);
                sqlite3_reset (updateStats);
            }
            sqlite3_finalize(updateStats);
            sqlite3_finalize(getTables);
        }

        bool SQLite3Driver::removeTorrent(uint8_t info_hash[20]) {
            // if non-dynamic, remove from table
            sqlite3_stmt *stmt;
            sqlite3_prepare(this->db, "DELETE FROM torrents WHERE info_hash=?", -1, &stmt, NULL);
            sqlite3_bind_blob(stmt, 1, info_hash, 20, NULL);
            sqlite3_step(stmt);
            sqlite3_finalize(stmt);

            // remove from stats
            sqlite3_stmt *rmS;
            if (sqlite3_prepare(this->db, "DELETE FROM stats WHERE info_hash=?", -1, &rmS, NULL) != SQLITE_OK)
            {
                sqlite3_finalize(rmS);
                return false;
            }
            sqlite3_bind_blob(rmS, 1, (const void*)info_hash, 20, NULL);
            sqlite3_step(rmS);
            sqlite3_finalize(rmS);

            // remove table
            string str = "DROP TABLE IF EXISTS 't";
            char buff [41];
            str += _to_hex_str(info_hash, buff);
            str += "'";

            sqlite3_exec(this->db, str.c_str(), NULL, NULL, NULL);

            return true;
        }

        bool SQLite3Driver::removePeer(uint8_t peer_id [20], uint8_t info_hash [20], uint32_t ip, uint16_t port) {
            string sql;
            char xHash [50];
            sqlite3_stmt *stmt;

            _to_hex_str (info_hash, xHash);

            sql += "DELETE FROM 't";
            sql += xHash;
            sql += "' WHERE ip=? AND port=? AND peer_id=?";

            sqlite3_prepare (this->db, sql.c_str(), sql.length(), &stmt, NULL);

            sqlite3_bind_blob(stmt, 0, (const void*)&ip, 4, NULL);
            sqlite3_bind_blob(stmt, 1, (const void*)&port, 2, NULL);
            sqlite3_bind_blob(stmt, 2, (const void*)peer_id, 20, NULL);

            sqlite3_step(stmt);

            sqlite3_finalize(stmt);

            return true;
        }

        static uint64_t _genCiD (uint32_t ip, uint16_t port) {
            uint64_t x;
            x = (time(NULL) / 3600) * port;	// x will probably overload.
            x = (ip ^ port);
            x <<= 16;
            x |= (~port);
            return x;
        }

        bool SQLite3Driver::genConnectionId (uint64_t *connectionId, uint32_t ip, uint16_t port) {
            *connectionId = _genCiD(ip, port);
            return true;
        }

        bool SQLite3Driver::verifyConnectionId(uint64_t cId, uint32_t ip, uint16_t port) {
            if (cId == _genCiD(ip, port))
                return true;
            else
                return false;
        }

        SQLite3Driver::~SQLite3Driver() {
            sqlite3_close(this->db);
        }
    };
};
