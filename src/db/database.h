/*
 * database.h
 *
 *  Created on: Nov 18, 2012
 *      Author: Naim
 *
 * This is just a API implementation; Actual management is done in the driver_*.c source.
 *
 *
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

int db_get_stats (dbConnection *, uint8_t [20], uint32_t *seeders, uint32_t *leechers, uint32_t *completed);

/**
 * Calculates Stats, Removes expired data.
 */
int db_cleanup (dbConnection *);

int db_remove_peer (dbConnection *, uint8_t hash [20], db_peerEntry *);

#endif /* DATABASE_H_ */
