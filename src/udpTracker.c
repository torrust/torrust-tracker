/*
 *	Copyright © 2012,2013 Naim A.
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
#include "settings.h"
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <stdio.h>

#define FLAG_RUNNING		0x01
#define UDP_BUFFER_SIZE		2048

#ifdef WIN32
static DWORD _thread_start (LPVOID arg);
static DWORD _maintainance_start (LPVOID arg);
#elif defined (linux)
static void* _thread_start (void *arg);
static void* _maintainance_start (void *arg);
#endif

static int _isTrue (char *str)
{
	int i,		// loop index
		len;	// string's length

	if (str == NULL)
		return -1;
	len = strlen (str);
	for (i = 0;i < len;i++)
	{
		if (str[i] >= 'A' && str[i] <= 'Z')
		{
			str[i] = (str[i] - 'A' + 'a');
		}
	}
	if (strcmp(str, "yes") == 0)
		return 1;
	if (strcmp(str, "no") == 0)
		return 0;
	if (strcmp(str, "true") == 0)
		return 1;
	if (strcmp(str, "false") == 0)
		return 0;
	if (strcmp(str, "1") == 0)
		return 1;
	if (strcmp(str, "0") == 0)
		return 0;
	return -1;
}

void UDPTracker_init (udpServerInstance *usi, Settings *settings)
{
	SettingClass *sc_tracker;
	uint8_t n_settings = 0;
	char *s_port, 			// port
		*s_threads,			// threads
		*s_allow_remotes,	// remotes allowed?
		*s_allow_iana_ip,	// IANA IPs allowed?
		*s_int_announce,	// announce interval
		*s_int_cleanup;		// cleanup interval

	sc_tracker = settings_get_class (settings, "tracker");

	s_port = settingclass_get(sc_tracker, "port");
	s_threads = settingclass_get(sc_tracker, "threads");
	s_allow_remotes = settingclass_get (sc_tracker, "allow_remotes");
	s_allow_iana_ip = settingclass_get (sc_tracker, "allow_iana_ips");
	s_int_announce = settingclass_get (sc_tracker, "announce_interval");
	s_int_cleanup = settingclass_get (sc_tracker, "cleanup_interval");

	if (_isTrue(s_allow_remotes) == 1)
		n_settings |= UDPT_ALLOW_REMOTE_IP;

	if (_isTrue(s_allow_iana_ip) != 0)
		n_settings |= UDPT_ALLOW_IANA_IP;

	usi->announce_interval = (s_int_announce == NULL ? 1800 : atoi (s_int_announce));
	usi->cleanup_interval = (s_int_cleanup == NULL ? 120 : atoi (s_int_cleanup));
	usi->port = (s_port == NULL ? 6969 : atoi (s_port));
	usi->thread_count = (s_threads == NULL ? 5 : atoi (s_threads)) + 1;

	usi->threads = (HANDLE*)malloc (sizeof(HANDLE) * usi->thread_count);

	usi->flags = 0;
	usi->conn = NULL;
	usi->settings = n_settings;
	usi->o_settings = settings;
}

void UDPTracker_destroy (udpServerInstance *usi)
{
	int i; // loop index

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

	for (i = 0;i < usi->thread_count;i++)
	{
#ifdef WIN32
		TerminateThread (usi->threads[0], 0x00);
#elif defined (linux)
		pthread_detach (usi->threads[i]);
		pthread_cancel (usi->threads[i]);
#endif
		printf ("Thread (%d/%u) terminated.\n", i + 1, usi->thread_count);
	}
	if (usi->conn != NULL)
		db_close(usi->conn);
	free (usi->threads);
}

int UDPTracker_start (udpServerInstance *usi)
{
	SOCKET sock;
	SOCKADDR_IN recvAddr;
	int r,		// saves results
		i,		// loop index
		yup;	// just to set TRUE
	char *dbname;// saves the Database name.

	sock = socket (AF_INET, SOCK_DGRAM, IPPROTO_UDP);
	if (sock == INVALID_SOCKET)
		return 1;

#ifdef WIN32
	recvAddr.sin_addr.S_un.S_addr = 0L;
#elif defined (linux)
	recvAddr.sin_addr.s_addr = 0L;
#endif
	recvAddr.sin_family = AF_INET;
	recvAddr.sin_port = m_hton16 (usi->port);

	yup = 1;
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

	usi->sock = sock;

	dbname = settings_get (usi->o_settings, "database", "file");
	if (dbname == NULL)
		dbname = "tracker.db";

	db_open(&usi->conn, dbname);

	usi->flags |= FLAG_RUNNING;

	// create maintainer thread.
#ifdef WIN32
		usi->threads[0] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)_maintainance_start, (LPVOID)usi, 0, NULL);
#elif defined (linux)
		printf("Starting maintenance thread (1/%u)...\n", usi->thread_count);
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
	int base;
	uint64_t x;

	base = time(NULL);
	base /= 3600;		// changes every day.

	x = base;
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
	int msg_sz,	// message size to send.
		i;		// copy loop
	char buff [1024];	// more than reasonable message size...

	error.action = m_hton32 (3);
	error.transaction_id = transactionID;
	error.message = msg;

	msg_sz = 4 + 4 + 1 + strlen(msg);

	memcpy(buff, &error, 8);
	for (i = 8;i <= msg_sz;i++)
	{
		buff[i] = msg[i - 8];
	}

	sendto(usi->sock, buff, msg_sz, 0, (SOCKADDR*)remote, sizeof(*remote));

	return 0;
}

static int _handle_connection (udpServerInstance *usi, SOCKADDR_IN *remote, char *data)
{
	ConnectionRequest *req;
	ConnectionResponse resp;

	req = (ConnectionRequest*)data;

	resp.action = m_hton32(0);
	resp.transaction_id = req->transaction_id;
	resp.connection_id = _get_connID(remote);

	sendto(usi->sock, (char*)&resp, sizeof(ConnectionResponse), 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	return 0;
}

static int _handle_announce (udpServerInstance *usi, SOCKADDR_IN *remote, char *data)
{
	AnnounceRequest *req;
	AnnounceResponse *resp;
	int q,		// peer counts
		bSize,	// message size
		i;		// loop index
	db_peerEntry *peers;
	int32_t seeders,
		leechers,
		completed;
	db_peerEntry pE;	// info for DB
	uint8_t buff [1028];	// Reasonable buffer size. (header+168 peers)

	req = (AnnounceRequest*)data;

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

	if ((usi->settings & UDPT_ALLOW_REMOTE_IP) == 0 && req->ip_address != 0)
	{
		_send_error (usi, remote, req->transaction_id, "Tracker doesn't allow remote IP's; Request ignored.");
		return 0;
	}

	// load peers
	q = 30;
	if (req->num_want >= 1)
		q = min (q, req->num_want);

	peers = (db_peerEntry*)malloc (sizeof(db_peerEntry) * q);

	db_load_peers(usi->conn, req->info_hash, peers, &q);
//	printf("%d peers found.\n", q);

	bSize = 20; // header is 20 bytes
	bSize += (6 * q); // + 6 bytes per peer.

	db_get_stats (usi->conn, req->info_hash, &seeders, &leechers, &completed);

	resp = (AnnounceResponse*)buff;
	resp->action = m_hton32(1);
	resp->interval = m_hton32 ( usi->announce_interval );
	resp->leechers = m_hton32(leechers);
	resp->seeders = m_hton32 (seeders);
	resp->transaction_id = req->transaction_id;

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

	}
	free (peers);
	sendto(usi->sock, (char*)buff, bSize, 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	// Add peer to list:
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
	ScrapeRequest *sR;
	int v,	// validation helper
		c,	// torrent counter
		i,	// loop counter
		j;	// loop counter
	uint8_t hash [20];
	char xHash [50];
	ScrapeResponse *resp;
	uint8_t buffer [1024];	// up to 74 torrents can be scraped at once (17*74+8) < 1024


	sR = (ScrapeRequest*)data;

	// validate request length:
	v = len - 16;
	if (v < 0 || v % 20 != 0)
	{
		_send_error (usi, remote, sR->transaction_id, "Bad scrape request.");
		return 0;
	}

	// get torrent count.
	c = v / 20;

	resp = (ScrapeResponse*)buffer;
	resp->action = m_hton32 (2);
	resp->transaction_id = sR->transaction_id;

	for (i = 0;i < c;i++)
	{
		int32_t s, c, l;
		int32_t *seeders,
			*completed,
			*leechers;

		for (j = 0; j < 20;j++)
			hash[j] = data[j + (i*20)+16];

		to_hex_str (hash, xHash);

		printf("\t%s\n", xHash);

		seeders = (int32_t*)&buffer[i*12+8];
		completed = (int32_t*)&buffer[i*12+12];
		leechers = (int32_t*)&buffer[i*12+16];

		db_get_stats (usi->conn, hash, &s, &l, &c);

		*seeders = m_hton32 (s);
		*completed = m_hton32 (c);
		*leechers = m_hton32 (l);
	}
	fflush (stdout);

	sendto (usi->sock, (const char*)buffer, sizeof(buffer), 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	return 0;
}

static int _isIANA_IP (uint32_t ip)
{
	uint8_t x = (ip % 256);
	if (x == 0 || x == 10 || x == 127 || x >= 224)
		return 1;
	return 0;
}

static int _resolve_request (udpServerInstance *usi, SOCKADDR_IN *remote, char *data, int r)
{
	ConnectionRequest *cR;
	uint32_t action;

	cR = (ConnectionRequest*)data;

	action = m_hton32(cR->action);

	if ((usi->settings & UDPT_ALLOW_IANA_IP) > 0)
	{
		if (_isIANA_IP (remote->sin_addr.s_addr))
		{
			return 0;	// Access Denied: IANA reserved IP.
		}
	}

	printf(":: %x:%u ACTION=%d\n", (unsigned int)remote->sin_addr.s_addr , remote->sin_port, action);

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
	udpServerInstance *usi;
	SOCKADDR_IN remoteAddr;
	int addrSz,
		r;
	char tmpBuff [UDP_BUFFER_SIZE];

	usi = arg;

	addrSz = sizeof (SOCKADDR_IN);


	while ((usi->flags & FLAG_RUNNING) > 0)
	{
		fflush(stdout);
		// peek into the first 12 bytes of data; determine if connection request or announce request.
		r = recvfrom(usi->sock, tmpBuff, UDP_BUFFER_SIZE, 0, (SOCKADDR*)&remoteAddr, (unsigned*)&addrSz);
		if (r <= 0)
			continue;	// bad request...
		r = _resolve_request(usi, &remoteAddr, tmpBuff, r);
	}

#ifdef linux
	pthread_exit (NULL);
#endif
	return 0;
}

#ifdef WIN32
static DWORD _maintainance_start (LPVOID arg)
#elif defined (linux)
static void* _maintainance_start (void *arg)
#endif
{
	udpServerInstance *usi;

	usi = (udpServerInstance *)arg;

	while ((usi->flags & FLAG_RUNNING) > 0)
	{
		db_cleanup (usi->conn);

#ifdef WIN32
		Sleep (usi->cleanup_interval * 1000);	// wait 2 minutes between every cleanup.
#elif defined (linux)
		sleep (usi->cleanup_interval);
#else
#error Unsupported OS.
#endif
	}

	return 0;
}
