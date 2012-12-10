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

#ifndef DATABASE_H_
#define DATABASE_H_

#include <stdint.h>

typedef struct dbConnection dbConnection;

/**
 * Opens a database connection.
 * @param pdb Pointer to database instance.
 * @param cStr Connection string for the active driver.
 * @return 0 on success; otherwise non-zero.
 */
int db_open (dbConnection **pdb, char *cStr);

/**
 * Closes the database connection.
 * @param db Database instance.
 * @return 0 on success; otherwise non-zero.
 */
int db_close (dbConnection *db);

typedef struct {
	uint8_t *peer_id;
	uint64_t downloaded;
	uint64_t uploaded;
	uint64_t left;

	uint32_t ip;	// currently only support IPv4.
	uint16_t port;
} db_peerEntry;

/**
 * Adds/Updates the list of peers.
 * @param db The database's instance.
 * @param hash The info_hash of the torrent.
 * @param pE Peer's information.
 * @return 0 on success; otherwise non-zero.
 */
int db_add_peer (dbConnection *db, uint8_t hash[20], db_peerEntry *pE);

/**
 * Loads peers for the requested torrent.
 * @param db Database instance.
 * @param hash The info_hash of the requested torrent.
 * @param lst A allocated array to store results in.
 * @param sZ in: The maximum amount of entries to load. out: Amount of loaded entries.
 * @return 0 on success; otherwise non-zero.
 */
int db_load_peers (dbConnection *db, uint8_t hash[20], db_peerEntry *lst, int *sZ);

/**
 * Gets stats for the requested torrent.
 * @param db The Database connection
 * @param hash info_hash of the torrent.
 * @param seeders Returns the Seeders for the requested torrent.
 * @param leechers Returns the Leechers for the requested torrent.
 * @param completed Returns the count of completed downloaded reported.
 * @return 0 on success, otherwise non-zero.
 */
int db_get_stats (dbConnection *db, uint8_t hash[20], int32_t *seeders, int32_t *leechers, int32_t *completed);

/**
 * Maintenance routine, Calculates stats & releases space from old entries.
 * @param db The database connection.
 * @return 0 on success; otherwise non-zero.
 */
int db_cleanup (dbConnection *db);

/**
 * Deletes a peer from the database.
 * @param db Database connection
 * @param hash info_hash of the torrent.
 * @param pE The peer's information.
 * @return 0 on success; otherwise non-zero.
 */
int db_remove_peer (dbConnection *db, uint8_t hash [20], db_peerEntry *pE);

#endif /* DATABASE_H_ */
