#include "database.h"

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

static void _to_hex_str (uint8_t hash[20], char *data)
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

	strcat(sql, "CREATE TABLE IF NOT EXISTS '");
	strcat(sql, hash);
	strcat (sql, "' (");

	strcat (sql, "peer_id blob(20) UNIQUE,"
			"ip blob(4),"
			"port blob(2),"
			"uploaded blob(8)," // uint64
			"downloaded blob(8),"
			"left blob(8),"
			"last_seen INTEGER(8)");

	strcat(sql, ")");

	// create table.
	char *err_msg;
	int r = sqlite3_exec(db, sql, NULL, NULL, &err_msg);
	printf("E:%s\n", err_msg);

	return r;
}

static void _db_setup (sqlite3 *db)
{
	sqlite3_exec(db, "CREATE TABLE stats ("
			"info_hash blob(20),"
			"completed INTEGER DEFAULT 0,"
			"leechers INTEGER DEFAULT 0,"
			"seeders INTEGER DEFAULT 0"
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
	_to_hex_str(info_hash, hash);

	_db_make_torrent_table(db->db, hash);

	sqlite3_stmt *stmt;

	char sql [1000];
	sql[0] = '\0';
	strcat(sql, "REPLACE INTO '");
	strcat(sql, hash);
	strcat(sql, "' (peer_id,ip,port,uploaded,downloaded,left,last_seen) VALUES (?,?,?,?,?,?,?)");

	sqlite3_prepare(db->db, sql, -1, &stmt, NULL);

	sqlite3_bind_blob(stmt, 1, pE->peer_id, 20, NULL);
	sqlite3_bind_blob(stmt, 2, &pE->ip, 4, NULL);
	sqlite3_bind_blob(stmt, 3, &pE->port, 2, NULL);
	sqlite3_bind_blob(stmt, 4, &pE->uploaded, 8, NULL);
	sqlite3_bind_blob(stmt, 5, &pE->downloaded, 8, NULL);
	sqlite3_bind_blob(stmt, 6, &pE->left, 8, NULL);
	sqlite3_bind_int64(stmt, 7, time(NULL));

	int r = sqlite3_step(stmt);
	sqlite3_finalize(stmt);

	return r;
}

int db_load_peers (dbConnection *db, uint8_t info_hash[20], db_peerEntry **lst, int *sZ)
{
	char sql [1000];
	sql[0] = '\0';

	char hash [50];
	_to_hex_str(info_hash, hash);

	strcat(sql, "SELECT ip,port FROM '");
	strcat(sql, hash);
	strcat(sql, "' LIMIT ?");

	sqlite3_stmt *stmt;
	sqlite3_prepare(db->db, sql, -1, &stmt, NULL);
	sqlite3_bind_int(stmt, 1, *sZ);

	int i = 0;
	int r;

	while (1)
	{
		r = sqlite3_step(stmt);
		if (r == SQLITE_ROW)
		{
			lst[i]->ip = sqlite3_column_int(stmt, 0);
			lst[i]->port = sqlite3_column_int(stmt, 1);
			i++;
		}
		else
			break;
	}

	sqlite3_finalize(stmt);

	*sZ = i;

	return 0;
}

int db_get_stats (dbConnection *db, uint8_t hash[20], uint32_t *seeders, uint32_t *leechers, uint32_t *completed)
{


	return 0;
}

int db_cleanup (dbConnection *db)
{
	printf("Cleanup...\n");

	sqlite3_stmt *stmt;

	return 0;
}
