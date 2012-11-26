
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
	usi->threads = malloc (sizeof(HANDLE) * threads);
	usi->flags = 0;
}

void UDPTracker_destroy (udpServerInstance *usi)
{
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
		pthread_create (&usi->threads[0], NULL, _maintainance_start, usi);
#endif

	for (i = 0;i < usi->thread_count; i++)
	{
		printf("Starting Thread %d of %u\n", (i + 1), usi->thread_count);
#ifdef WIN32
		usi->threads[i] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)_thread_start, (LPVOID)usi, 0, NULL);
#elif defined (linux)
		pthread_create (&usi->threads[i], NULL, _thread_start, usi);
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

	printf("ERROR SENT\n");
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

static int _is_good_peer (uint32_t ip, uint16_t port)
{
	SOCKADDR_IN addr;
	addr.sin_family = AF_INET;
#ifdef WIN32
	addr.sin_addr.S_un.S_addr = htonl( ip );
#elif defined (linux)
	addr.sin_addr.s_addr = htonl( ip );
#endif
	addr.sin_port = htons (port);

	SOCKET cli = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
	if (cli == INVALID_SOCKET)
		return 1;
	if (connect(cli, (SOCKADDR*)&addr, sizeof(SOCKADDR_IN)) == SOCKET_ERROR)
	{
		closesocket (cli);
		return 1;
	}
	printf ("Client Verified.\n");
	closesocket (cli);
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
	if (req->ip_address == 0) // default
	{
		req->ip_address = m_hton32 (remote->sin_addr.s_addr);
	}

//	if (_is_good_peer(req->ip_address, req->port) != 0)
//	{
//		_send_error (usi, remote, req->transaction_id, "Couldn't verify your client.");
//		return 0;
//	}

	// load peers
	int q = 30;
	if (req->num_want >= 1)
		q = min (q, req->num_want);

	db_peerEntry *peers = NULL;
	int bSize = 20; // header is 20 bytes

	if (req->event == 3) // stopped; they don't need anymore peers!
	{
		q = 0; // don't need any peers!
	}
	else
	{
		peers = malloc (sizeof(db_peerEntry) * q);
		db_load_peers(usi->conn, req->info_hash, peers, &q);
	}

	bSize += (6 * q); // + 6 bytes per peer (ip->4, port->2).

	uint32_t seeders, leechers, completed;
	db_get_stats (usi->conn, req->info_hash, &seeders, &leechers, &completed);

	uint8_t buff [bSize];
	AnnounceResponse *resp = (AnnounceResponse*)buff;
	resp->action = m_hton32(1);
	resp->interval = m_hton32 ( 1800 );
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

	if (peers != NULL)
		free (peers);

	sendto(usi->sock, (char*)buff, bSize, 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));


	// Add peer to list:
	db_peerEntry pE;
	pE.downloaded = req->downloaded;
	pE.uploaded = req->uploaded;
	pE.left = req->left;
	pE.peer_id = req->peer_id;
	pE.ip = req->ip_address;
	pE.port = req->port;

	if (req->event == 3) // stopped
	{
		// just remove client from swarm, and return empty peer list...
		db_remove_peer(usi->conn, req->info_hash, &pE);
		return 0;
	}

	db_add_peer(usi->conn, req->info_hash,  &pE);

	return 0;
}

static int _handle_scrape (udpServerInstance *usi, SOCKADDR_IN *remote, char *data, int len)
{
	ScrapeRequest *sR = (ScrapeRequest*)data;

//	_send_error (usi, remote, sR->transaction_id, "Scrape wasn't implemented yet!");

	ScrapeResponse resp;
	resp.resp_part = NULL;
	resp.action = 2;
	resp.transaction_id = sR->transaction_id;

	sendto (usi->sock, (const char*)&resp, 8, 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	return 0;
}

static int _resolve_request (udpServerInstance *usi, SOCKADDR_IN *remote, char *data, int r)
{
	ConnectionRequest *cR;
	cR = (ConnectionRequest*)data;

	uint32_t action = m_hton32(cR->action);

//	printf(":: %x:%u ACTION=%d\n", remote->sin_addr.s_addr , remote->sin_port, action);

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

	char *tmpBuff = malloc (UDP_BUFFER_SIZE); // 98 is the maximum request size.

	while ((usi->flags & FLAG_RUNNING) > 0)
	{
		fflush(stdout);
		// peek into the first 12 bytes of data; determine if connection request or announce request.
		r = recvfrom(usi->sock, tmpBuff, UDP_BUFFER_SIZE, 0, (SOCKADDR*)&remoteAddr, &addrSz);
//		printf("RECV:%d\n", r);
		r = _resolve_request(usi, &remoteAddr, tmpBuff, r);
//		printf("R=%d\n", r);
	}

	free (tmpBuff);

	return 0;
}

#ifdef WIN32
static DWORD _maintainance_start (LPVOID arg)
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
