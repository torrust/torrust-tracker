
#include <winsock2.h>
#include <windows.h>
#include "udpTracker.h"
#include "tools.h"
#include <stdlib.h>
#include <time.h>
#include <stdio.h>
#include <winerror.h>

#define FLAG_RUNNING	0x01
#define UDP_BUFFER_SIZE	256

static DWORD _thread_start (LPVOID);

void UDPTracker_init (udpServerInstance *usi, uint16_t port, uint8_t threads)
{
	usi->port = port;
	usi->thread_count = threads;
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

	recvAddr.sin_addr.S_un.S_addr = 0L;
	recvAddr.sin_family = AF_INET;
	recvAddr.sin_port = htons (usi->port);

	BOOL yup = TRUE;
	setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, (const char*)&yup, sizeof(BOOL));

	r = bind (sock, (SOCKADDR*)&recvAddr, sizeof(SOCKADDR_IN));

	if (r == SOCKET_ERROR)
	{
		closesocket (sock);
		return 2;
	}

	printf("SOCK=%d\n", sock);
	usi->sock = sock;

	db_open(&usi->conn, "tracker.db");

	usi->flags |= FLAG_RUNNING;
	int i;
	for (i = 0;i < usi->thread_count; i++)
	{
		printf("Starting Thread %d of %u\n", (i + 1), usi->thread_count);
		usi->threads[i] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)_thread_start, (LPVOID)usi, 0, NULL);
	}

	return 0;
}

static uint64_t _get_connID (SOCKADDR_IN *remote)
{
	int base = time(NULL);
	base /= 3600;		// changes every day.

	uint64_t x = base;
	x += remote->sin_addr.S_un.S_addr;
	return x;
}

static int _send_error (udpServerInstance *usi, SOCKADDR_IN *remote, uint32_t transactionID, char *msg)
{
	struct udp_error_response error;
	error.action = htonl(3);
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
	resp.action = htonl(0);
	resp.transaction_id = req->transaction_id;
	resp.connection_id = _get_connID(remote);

	int r = sendto(usi->sock, (char*)&resp, sizeof(ConnectionResponse), 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

	printf("_h_c=%d\n", r);

	return 0;
}

static int _handle_announce (udpServerInstance *usi, SOCKADDR_IN *remote, char *data)
{
	AnnounceRequest *req = (AnnounceRequest*)data;

	if (req->connection_id != _get_connID(remote))
	{
		printf("ConnID mismatch.\n");
		return 1;
	}

	db_peerEntry pE;
	pE.downloaded = req->downloaded;
	pE.uploaded = req->uploaded;
	pE.left = req->left;
	pE.peer_id = req->peer_id;
	pE.ip = req->ip_address;
	pE.port = req->port;

	db_add_peer(usi->conn, req->info_hash,  &pE);


//	_send_error(usi, remote, req->transaction_id, "Not Implemented :-(.");

	int q = 30;
	if (req->num_want >= 1)
		q = min (q, req->num_want);

	db_peerEntry *peers = malloc (sizeof(db_peerEntry) * q);
	db_load_peers(usi->conn, req->info_hash, &peers, &q);
	printf("%d peers found.\n", q);

	int bSize = 20; // header is 20 bytes
	bSize += (6 * q); // + 6 bytes per peer.

	uint8_t buff [bSize];

	AnnounceResponse *resp = (AnnounceResponse*)buff;
	resp->action = htonl(1);
	resp->interval = htonl ( 1800 );
	resp->leechers = htonl( 1);
	resp->seeders = 0;
	resp->transaction_id = req->transaction_id;

	int i;

	for (i = 0;i < q;i++)
	{
		int x = i * 6;
		// network byte order!!!
		buff[20 + x] = ((peers[i].ip & (0xff << 24)) >> 24);
		buff[21 + x] = ((peers[i].ip & (0xff << 16)) >> 16);
		buff[22 + x] = ((peers[i].ip & (0xff << 8)) >> 8);
		buff[23 + x] = (peers[i].ip & 0xff);

		buff[24 + x] = ((peers[i].port & (0xff << 8)) >> 8);
		buff[25 + x] = (peers[i].port & 0xff);

		printf("%u.%u.%u.%u:%u\n", buff[20 + x], buff[21 + x], buff[22 + x], buff[23 + x], peers[i].port);
	}

	free (peers);

	return sendto(usi->sock, (char*)buff, bSize, 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));
}

// returns 1 if connection request. returns 2 if announce. returns 3 if scrape.
static int _resolve_request (udpServerInstance *usi, SOCKADDR_IN *remote, char *data)
{
	ConnectionRequest *cR;
	cR = (ConnectionRequest*)data;

	uint32_t action = htonl(cR->action);

	printf("ACTION=%d\n", action);

	if (action == 0)
		return _handle_connection(usi, remote, data);
	else if (action == 1)
		return _handle_announce(usi, remote, data);
	else
	{
		_send_error(usi, remote, cR->transaction_id, "Method not implemented.");
		return -1;
	}
}

static DWORD _thread_start (LPVOID arg)
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
		printf("RECV:%d\n", r);
		r = _resolve_request(usi, &remoteAddr, tmpBuff);
		printf("R=%d\n", r);
	}

	free (tmpBuff);

	return 0;
}
