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

#include "multiplatform.h"
#include "udpTracker.h"
#include "tools.h"
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <stdio.h>

#define FLAG_RUNNING		0x01
#define UDP_BUFFER_SIZE		2048
#define CLEANUP_INTERVAL	20
#define TRACKER_INTERVAL	60 // normally 1800

#ifdef WIN32
static DWORD _thread_start (LPVOID arg);
static DWORD _maintainance_start (LPVOID arg);
#elif defined (linux)
static void* _thread_start (void *arg);
static void* _maintainance_start (void *arg);
#endif

void UDPTracker_init (udpServerInstance *usi, uint16_t port, uint8_t threads)
{
	usi->port = port;
	usi->thread_count = threads + 1;
	usi->threads = malloc (sizeof(HANDLE) * usi->thread_count);
	usi->flags = 0;
	usi->conn = NULL;
}

void UDPTracker_destroy (udpServerInstance *usi)
{
	usi->flags = (!(FLAG_RUNNING)) & usi->flags;


	// drop listener connection to continue thread loops.
	// wait for request to finish (1 second max; allot of time for a computer!).

#ifdef linux
	close (usi->sock);

	sleep (1);
#elif defined (WIN32)
	closesocket (usi->sock);

	Sleep (1000);
#endif

	int i;
	for (i = 0;i < usi->thread_count;i++)
	{
		pthread_detach (usi->threads[i]);
		pthread_cancel (usi->threads[i]);
		printf ("Thread (%d/%u) terminated.\n", i + 1, usi->thread_count);
	}
	if (usi->conn != NULL)
		db_close(usi->conn);
	free (usi->threads);
}


int UDPTracker_start (udpServerInstance *usi)
{
	SOCKET sock = socket (AF_INET, SOCK_DGRAM, IPPROTO_UDP);
	if (sock == INVALID_SOCKET)
		return 1;

	int r;

	SOCKADDR_IN recvAddr;
#ifdef WIN32
	recvAddr.sin_addr.S_un.S_addr = 0L;
#elif defined (linux)
	recvAddr.sin_addr.s_addr = 0L;
#endif
	recvAddr.sin_family = AF_INET;
	recvAddr.sin_port = m_hton16 (usi->port);

	int yup = 1;
	setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, (const char*)&yup, 1);

	r = bind (sock, (SOCKADDR*)&recvAddr, sizeof(SOCKADDR_IN));

	if (r == SOCKET_ERROR)
	{
#ifdef WIN32
		closesocket (sock);
#elif defined (linux)
		close (sock);
#endif
		return 2;
	}

	printf("SOCK=%d\n", sock);
	usi->sock = sock;

	db_open(&usi->conn, "tracker.db");

	usi->flags |= FLAG_RUNNING;
	int i;

	// create maintainer thread.
#ifdef WIN32
		usi->threads[0] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)_maintainance_start, (LPVOID)usi, 0, NULL);
#elif defined (linux)
		printf("Starting maintenance thread (1/6)...\n");
		pthread_create (&usi->threads[0], NULL, _maintainance_start, usi);
#endif

	for (i = 1;i < usi->thread_count; i++)
	{
		printf("Starting Thread (%d/%u)\n", (i + 1), usi->thread_count);
#ifdef WIN32
		usi->threads[i] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)_thread_start, (LPVOID)usi, 0, NULL);
#elif defined (linux)
		pthread_create (&(usi->threads[i]), NULL, _thread_start, usi);
#endif
	}

	return 0;
}

static uint64_t _get_connID (SOCKADDR_IN *remote)
{
	int base = time(NULL);
	base /= 3600;		// changes every day.

	uint64_t x = base;
#ifdef WIN32
	x += remote->sin_addr.S_un.S_addr;
#elif defined (linux)
	x += remote->sin_addr.s_addr;
#endif
	return x;
}

static int _send_error (udpServerInstance *usi, SOCKADDR_IN *remote, uint32_t transactionID, char *msg)
{
	struct udp_error_response error;
	error.action = m_hton32 (3);
	error.transaction_id = transactionID;
	error.message = msg;

	int msg_sz = 4 + 4 + 1 + strlen(msg);

	char buff [msg_sz];
	memcpy(buff, &error, 8);
	int i;
	for (i = 8;i <= msg_sz;i++)
	{
		buff[i] = msg[i - 8];
	}

	sendto(usi->sock, buff, msg_sz, 0, (SOCKADDR*)remote, sizeof(*remote));

	return 0;
}

static int _handle_connection (udpServerInstance *usi, SOCKADDR_IN *remote, char *data)
{
	ConnectionRequest *req = (ConnectionRequest*)data;

	ConnectionResponse resp;
	resp.action = m_hton32(0);
	resp.transaction_id = req->transaction_id;
	resp.connection_id = _get_connID(remote);

	sendto(usi->sock, (char*)&resp, sizeof(ConnectionResponse), 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	return 0;
}

static int _handle_announce (udpServerInstance *usi, SOCKADDR_IN *remote, char *data)
{
	AnnounceRequest *req = (AnnounceRequest*)data;

	if (req->connection_id != _get_connID(remote))
	{
		return 1;
	}

	// change byte order:
	req->port = m_hton16 (req->port);
	req->ip_address = m_hton32 (req->ip_address);
	req->downloaded = m_hton64 (req->downloaded);
	req->event = m_hton32 (req->event);	// doesn't really matter for this tracker
	req->uploaded = m_hton64 (req->uploaded);
	req->num_want = m_hton32 (req->num_want);
	req->left = m_hton64 (req->left);


	// load peers
	int q = 30;
	if (req->num_want >= 1)
		q = min (q, req->num_want);

	db_peerEntry *peers = malloc (sizeof(db_peerEntry) * q);

	db_load_peers(usi->conn, req->info_hash, peers, &q);
//	printf("%d peers found.\n", q);

	int bSize = 20; // header is 20 bytes
	bSize += (6 * q); // + 6 bytes per peer.

	int32_t seeders, leechers, completed;
	db_get_stats (usi->conn, req->info_hash, &seeders, &leechers, &completed);

	uint8_t buff [bSize];
	AnnounceResponse *resp = (AnnounceResponse*)buff;
	resp->action = m_hton32(1);
	resp->interval = m_hton32 ( TRACKER_INTERVAL );
	resp->leechers = m_hton32(leechers);
	resp->seeders = m_hton32 (seeders);
	resp->transaction_id = req->transaction_id;

	int i;
	for (i = 0;i < q;i++)
	{
		int x = i * 6;
		// network byte order!!!

		// IP
		buff[20 + x] = ((peers[i].ip & (0xff << 24)) >> 24);
		buff[21 + x] = ((peers[i].ip & (0xff << 16)) >> 16);
		buff[22 + x] = ((peers[i].ip & (0xff << 8)) >> 8);
		buff[23 + x] = (peers[i].ip & 0xff);

		// port
		buff[24 + x] = ((peers[i].port & (0xff << 8)) >> 8);
		buff[25 + x] = (peers[i].port & 0xff);

//		printf("%u.%u.%u.%u:%u\n", buff[20 + x], buff[21 + x], buff[22 + x], buff[23 + x], peers[i].port);
	}
	free (peers);
	sendto(usi->sock, (char*)buff, bSize, 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	// Add peer to list:
	db_peerEntry pE;
	pE.downloaded = req->downloaded;
	pE.uploaded = req->uploaded;
	pE.left = req->left;
	pE.peer_id = req->peer_id;
	if (req->ip_address == 0) // default
	{
		pE.ip = m_hton32 (remote->sin_addr.s_addr);
	}
	else
	{
		pE.ip = req->ip_address;
	}
	pE.port = req->port;
	db_add_peer(usi->conn, req->info_hash,  &pE);

	return 0;
}

static int _handle_scrape (udpServerInstance *usi, SOCKADDR_IN *remote, char *data, int len)
{
	ScrapeRequest *sR = (ScrapeRequest*)data;

//	_send_error (usi, remote, sR->transaction_id, "Scrape wasn't implemented yet!");

	// validate request length:
	int v = len - 16;
	if (v < 0 || v % 20 != 0)
	{
//		printf("BAD SCRAPE: len=%d\n", len);
		_send_error (usi, remote, sR->transaction_id, "Bad scrape request.");
		return 0;
	}

	// get torrent count.
	int c = v / 20;

	uint8_t hash [20];
	char xHash [50];

	uint8_t buffer [8 + (12 * c)];
	ScrapeResponse *resp = (ScrapeResponse*)buffer;
	resp->action = m_hton32 (2);
	resp->transaction_id = sR->transaction_id;

	int i, j;
//	printf("Scrape: (%d) \n", c);
	for (i = 0;i < c;i++)
	{
		for (j = 0; j < 20;j++)
			hash[j] = data[j + (i*20)+16];

		to_hex_str (hash, xHash);

		printf("\t%s\n", xHash);

		int32_t *seeders = (int32_t*)&buffer[i*12+8];
		int32_t *completed = (int32_t*)&buffer[i*12+12];
		int32_t *leechers = (int32_t*)&buffer[i*12+16];

		int32_t s, c, l;

		db_get_stats (usi->conn, hash, &s, &l, &c);

		*seeders = m_hton32 (s);
		*completed = m_hton32 (c);
		*leechers = m_hton32 (l);
	}
	fflush (stdout);

	sendto (usi->sock, buffer, sizeof(buffer), 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	return 0;
}

static int _resolve_request (udpServerInstance *usi, SOCKADDR_IN *remote, char *data, int r)
{
	ConnectionRequest *cR;
	cR = (ConnectionRequest*)data;

	uint32_t action = m_hton32(cR->action);

	printf(":: %x:%u ACTION=%d\n", remote->sin_addr.s_addr , remote->sin_port, action);

	if (action == 0 && r >= 16)
		return _handle_connection(usi, remote, data);
	else if (action == 1 && r >= 98)
		return _handle_announce(usi, remote, data);
	else if (action == 2)
		return _handle_scrape (usi, remote, data, r);
	else
	{
		printf("E: action=%d; r=%d\n", action, r);
		_send_error(usi, remote, cR->transaction_id, "Tracker couldn't understand Client's request.");
		return -1;
	}

	return 0;
}

#ifdef WIN32
static DWORD _thread_start (LPVOID arg)
#elif defined (linux)
static void* _thread_start (void *arg)
#endif
{
	udpServerInstance *usi = arg;

	SOCKADDR_IN remoteAddr;
	int addrSz = sizeof (SOCKADDR_IN);
	int r;

//	char *tmpBuff = malloc (UDP_BUFFER_SIZE); // 98 is the maximum request size.

	char tmpBuff [UDP_BUFFER_SIZE];

	while ((usi->flags & FLAG_RUNNING) > 0)
	{
		fflush(stdout);
		// peek into the first 12 bytes of data; determine if connection request or announce request.
		r = recvfrom(usi->sock, tmpBuff, UDP_BUFFER_SIZE, 0, (SOCKADDR*)&remoteAddr, (unsigned*)&addrSz);
		if (r <= 0)
			continue;	// bad request...
//		printf("RECV:%d\n", r);
		r = _resolve_request(usi, &remoteAddr, tmpBuff, r);
//		printf("R=%d\n", r);
	}

//	free (tmpBuff);

	pthread_exit (NULL);
	return 0;
}

#ifdef WIN32
static DWORD _maintainance_start (LPVOID arg);
#elif defined (linux)
static void* _maintainance_start (void *arg)
#endif
{
	udpServerInstance *usi = (udpServerInstance *)arg;

	while ((usi->flags & FLAG_RUNNING) > 0)
	{
		db_cleanup (usi->conn);

#ifdef WIN32
		Sleep (CLEANUP_INTERVAL * 1000);	// wait 2 minutes between every cleanup.
#elif defined (linux)
		sleep (CLEANUP_INTERVAL);
#else
#error Unsupported OS.
#endif
	}

	return 0;
}
