/*
 *	Copyright © 2012 Naim A.
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

#include "database.h"
#include "../multiplatform.h"
#include "../tools.h"
#include <sqlite3.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

struct dbConnection
{
	sqlite3 *db;
	HANDLE janitor;
};

static const char hexadecimal[] = "0123456789abcdef";

void _to_hex_str (const uint8_t *hash, char *data)
{
	int i;
	for (i = 0;i < 20;i++)
	{
		data[i * 2] = hexadecimal[hash[i] / 16];
		data[i * 2 + 1] = hexadecimal[hash[i] % 16];
	}
	data[40] = '\0';
}

static int _db_make_torrent_table (sqlite3 *db, char *hash)
{
	char sql [2000];
	sql[0] = '\0';

	strcat(sql, "CREATE TABLE IF NOT EXISTS 't");
	strcat(sql, hash);
	strcat (sql, "' (");

	strcat (sql, "peer_id blob(20),"
			"ip blob(4),"
			"port blob(2),"
			"uploaded blob(8)," // uint64
			"downloaded blob(8),"
			"left blob(8),"
			"last_seen INT DEFAULT 0");

	strcat(sql, ", CONSTRAINT c1 UNIQUE (ip,port) ON CONFLICT REPLACE)");

	// create table.
	char *err_msg;
	int r = sqlite3_exec(db, sql, NULL, NULL, &err_msg);
	printf("E:%s\n", err_msg);

	return r;
}

static void _db_setup (sqlite3 *db)
{
	sqlite3_exec(db, "CREATE TABLE stats ("
			"info_hash blob(20) UNIQUE,"
			"completed INTEGER DEFAULT 0,"
			"leechers INTEGER DEFAULT 0,"
			"seeders INTEGER DEFAULT 0,"
			"last_mod INTEGER DEFAULT 0"
			")", NULL, NULL, NULL);
}

int db_open (dbConnection **db, char *cStr)
{
	FILE *f = fopen (cStr, "rb");
	int doSetup = 0;
	if (f == NULL)
		doSetup = 1;
	else
		fclose (f);

	*db = malloc (sizeof(struct dbConnection));
	int r = sqlite3_open (cStr, &((*db)->db));
	if (doSetup)
		_db_setup((*db)->db);



	return r;
}

int db_close (dbConnection *db)
{
	int r = sqlite3_close(db->db);
	free (db);
	return r;
}

int db_add_peer (dbConnection *db, uint8_t info_hash[20], db_peerEntry *pE)
{
	char xHash [50]; // we just need 40 + \0 = 41.

	char *hash = xHash;
	to_hex_str(info_hash, hash);

	_db_make_torrent_table(db->db, hash);

	sqlite3_stmt *stmt;

	char sql [1000];
	sql[0] = '\0';
	strcat(sql, "REPLACE INTO 't");
	strcat(sql, hash);
	strcat(sql, "' (peer_id,ip,port,uploaded,downloaded,left,last_seen) VALUES (?,?,?,?,?,?,?)");

//	printf("IP->%x::%u\n", pE->ip, pE->port);

	sqlite3_prepare(db->db, sql, -1, &stmt, NULL);

	sqlite3_bind_blob(stmt, 1, (void*)pE->peer_id, 20, NULL);
	sqlite3_bind_blob(stmt, 2, (void*)&pE->ip, 4, NULL);
	sqlite3_bind_blob(stmt, 3, (void*)&pE->port, 2, NULL);
	sqlite3_bind_blob(stmt, 4, (void*)&pE->uploaded, 8, NULL);
	sqlite3_bind_blob(stmt, 5, (void*)&pE->downloaded, 8, NULL);
	sqlite3_bind_blob(stmt, 6, (void*)&pE->left, 8, NULL);
	sqlite3_bind_int(stmt, 7, time(NULL));

	int r = sqlite3_step(stmt);
	sqlite3_finalize(stmt);

	strcpy(sql, "REPLACE INTO stats (info_hash,last_mod) VALUES (?,?)");
	sqlite3_prepare (db->db, sql, -1, &stmt, NULL);
	sqlite3_bind_blob (stmt, 1, hash, 20, NULL);
	sqlite3_bind_int (stmt, 2, time(NULL));
	sqlite3_step (stmt);
	sqlite3_finalize (stmt);

	return r;
}

int db_load_peers (dbConnection *db, uint8_t info_hash[20], db_peerEntry *lst, int *sZ)
{
	char sql [1000];
	sql[0] = '\0';

	char hash [50];
	to_hex_str(info_hash, hash);

	strcat(sql, "SELECT ip,port FROM 't");
	strcat(sql, hash);
	strcat(sql, "' LIMIT ?");

	sqlite3_stmt *stmt;
	sqlite3_prepare(db->db, sql, -1, &stmt, NULL);
	sqlite3_bind_int(stmt, 1, *sZ);

	int i = 0;
	int r;

	while (*sZ > i)
	{
		r = sqlite3_step(stmt);
		if (r == SQLITE_ROW)
		{
			const char *ip = (const char*)sqlite3_column_blob (stmt, 0);
			const char *port = (const char*)sqlite3_column_blob (stmt, 1);

			memcpy(&lst[i].ip, ip, 4);
			memcpy(&lst[i].port, port, 2);

			i++;
		}
		else
			break;
	}

	printf("%d Clients Dumped.\n", i);

	sqlite3_finalize(stmt);

	*sZ = i;

	return 0;
}

int db_get_stats (dbConnection *db, uint8_t hash[20], int32_t *seeders, int32_t *leechers, int32_t *completed)
{
	*seeders = 0;
	*leechers = 0;
	*completed = 0;

	const char sql[] = "SELECT seeders,leechers,completed FROM 'stats' WHERE info_hash=?";

	sqlite3_stmt *stmt;
	sqlite3_prepare (db->db, sql, -1, &stmt, NULL);
	sqlite3_bind_blob (stmt, 1, (void*)hash, 20, NULL);

	if (sqlite3_step(stmt) == SQLITE_ROW)
	{
		*seeders = sqlite3_column_int (stmt, 0);
		*leechers = sqlite3_column_int (stmt, 1);
		*completed = sqlite3_column_int (stmt, 2);
	}

	sqlite3_finalize (stmt);

	return 0;
}

int db_cleanup (dbConnection *db)
{
	return 0;	// TODO: Fix problems and than allow use of this function.
	printf("Cleanup...\n");

	sqlite3_stmt *stmt;

	int timeframe = time(NULL);

	// remove "dead" torrents (non-active for two hours).
	const char sql[] = "SELECT info_hash FROM stats WHERE last_mod<?";
	sqlite3_prepare (db->db, sql, -1, &stmt, NULL);
	sqlite3_bind_int (stmt, 1, timeframe - 7200);
	char hash [50], temp [1000];

	while (sqlite3_step(stmt) == SQLITE_ROW)
	{
		to_hex_str(sqlite3_column_blob(stmt, 0), hash);

		// drop table:
		strcpy(temp, "DROP TABLE IF EXISTS 't");
		strcat(temp, hash);
		strcat(temp, "'");
		sqlite3_exec(db->db, temp, NULL, NULL, NULL);
	}
	sqlite3_finalize (stmt);

	// update 'dead' torrents
	sqlite3_prepare(db->db, "UPDATE stats SET seeders=0,leechers=0 WHERE last_mod<?", -1, &stmt, NULL);
	sqlite3_bind_int (stmt, 1, timeframe - 7200);
	sqlite3_step (stmt);
	sqlite3_finalize (stmt);

	// update regular torrents.
	sqlite3_prepare(db->db, "SELECT info_hash FROM stats WHERE last_mod>=?", -1, &stmt, NULL);
	sqlite3_bind_int (stmt, 1, timeframe - 7200);

	uint32_t leechers, seeders;
	sqlite3_stmt *sTmp, *uStat;

	sqlite3_prepare (db->db, "UPDATE stats SET seeders=?,leechers=?,last_mod=? WHERE info_hash=?", -1, &uStat, NULL);

	while (sqlite3_step(stmt) == SQLITE_ROW)
	{
		uint8_t *binHash = (uint8_t*)sqlite3_column_blob(stmt, 0);
		to_hex_str (binHash, hash);

		// total users...
		strcpy (temp, "SELECT COUNT(*) FROM 't");
		strcat (temp, hash);
		strcat (temp, "'");

		sqlite3_prepare (db->db, temp, -1, &sTmp, NULL);
		if (sqlite3_step(sTmp) == SQLITE_ROW)
		{
			leechers = sqlite3_column_int (sTmp, 0);
		}
		sqlite3_finalize (sTmp);

		// seeders...
		strcpy (temp, "SELECT COUNT(*) FROM 't");
		strcat (temp, hash);
		strcat (temp, "' WHERE left=0");

		sqlite3_prepare (db->db, temp, -1, &sTmp, NULL);
		if (sqlite3_step(sTmp) == SQLITE_ROW)
		{
			seeders = sqlite3_column_int (sTmp, 0);
		}
		sqlite3_finalize (sTmp);

		leechers -= seeders;

		sqlite3_bind_int (uStat, 1, seeders);
		sqlite3_bind_int (uStat, 2, leechers);
		sqlite3_bind_int (uStat, 3, timeframe);
		sqlite3_bind_blob (uStat, 4, binHash, 20, NULL);
		sqlite3_step (uStat);
		sqlite3_reset (uStat);

		printf("%s: %d seeds/%d leechers;\n", hash, seeders, leechers);
	}
	sqlite3_finalize (stmt);

	sqlite3_finalize (stmt);

	return 0;
}

int db_remove_peer (dbConnection *db, uint8_t hash[20], db_peerEntry *pE)
{
	char sql [1000];
	char xHash [50];

	_to_hex_str (hash, xHash);

	strcpy (sql, "DELETE FROM 't");
	strcat (sql, xHash);
	strcat (sql, "' WHERE ip=? AND port=? AND peer_id=?");

	sqlite3_stmt *stmt;

	sqlite3_prepare (db->db, sql, -1, &stmt, NULL);

	sqlite3_bind_blob(stmt, 0, (const void*)&pE->ip, 4, NULL);
	sqlite3_bind_blob(stmt, 1, (const void*)&pE->port, 2, NULL);
	sqlite3_bind_blob(stmt, 2, (const void*)pE->peer_id, 20, NULL);

	sqlite3_step(stmt);

	sqlite3_finalize(stmt);

	return 0;
}
