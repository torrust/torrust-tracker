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

int db_open (dbConnection **, char *cStr);
int db_close (dbConnection *);

typedef struct {
	uint8_t *peer_id;
	uint64_t downloaded;
	uint64_t uploaded;
	uint64_t left;

	uint32_t ip;	// currently only support IPv4.
	uint16_t port;
} db_peerEntry;

// adds a peer to the torrent's list.
int db_add_peer (dbConnection *, uint8_t [20], db_peerEntry*);

/*
 * lst: pointer to an array whose maximum size is passed to sZ.
 * sZ returns the amount of peers returned.
 */
int db_load_peers (dbConnection *, uint8_t [20], db_peerEntry *lst, int *sZ);

int db_get_stats (dbConnection *, uint8_t [20], int32_t *seeders, int32_t *leechers, int32_t *completed);

/**
 * Calculates Stats, Removes expired data.
 */
int db_cleanup (dbConnection *);

int db_remove_peer (dbConnection *, uint8_t hash [20], db_peerEntry *);

#endif /* DATABASE_H_ */
