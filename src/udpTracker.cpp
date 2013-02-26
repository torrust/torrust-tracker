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
#include "udpTracker.hpp"
#include "tools.h"
#include <stdlib.h>
#include <time.h>

#include <iostream>
using namespace std;

#define UDP_BUFFER_SIZE		2048

namespace UDPT
{
	inline static int _isTrue (string str)
	{
		int i,		// loop index
			len;	// string's length

		if (str == "")
			return -1;
		len = str.length();
		for (i = 0;i < len;i++)
		{
			if (str[i] >= 'A' && str[i] <= 'Z')
			{
				str[i] = (str[i] - 'A' + 'a');
			}
		}
		if (str.compare ("yes") == 0)
			return 1;
		if (str.compare ("no") == 0)
			return 0;
		if (str.compare("true") == 0)
			return 1;
		if (str.compare ("false") == 0)
			return 0;
		if (str.compare("1") == 0)
			return 1;
		if (str.compare ("0") == 0)
			return 0;
		return -1;
	}

	UDPTracker::UDPTracker (Settings *settings)
	{
		Settings::SettingClass *sc_tracker;
		uint8_t n_settings = 0;
		string s_port, 			// port
			s_threads,			// threads
			s_allow_remotes,	// remotes allowed?
			s_allow_iana_ip,	// IANA IPs allowed?
			s_int_announce,	// announce interval
			s_int_cleanup;		// cleanup interval

		sc_tracker = settings->getClass("tracker");

		s_port = sc_tracker->get ("port");
		s_threads = sc_tracker->get ("threads");
		s_allow_remotes = sc_tracker->get ("allow_remotes");
		s_allow_iana_ip = sc_tracker->get ("allow_iana_ips");
		s_int_announce = sc_tracker->get ("announce_interval");
		s_int_cleanup = sc_tracker-> get ("cleanup_interval");

		if (_isTrue(s_allow_remotes) == 1)
			n_settings |= UDPT_ALLOW_REMOTE_IP;

		if (_isTrue(s_allow_iana_ip) != 0)
			n_settings |= UDPT_ALLOW_IANA_IP;

		this->announce_interval = (s_int_announce == "" ? 1800 : atoi (s_int_announce.c_str()));
		this->cleanup_interval = (s_int_cleanup == "" ? 120 : atoi (s_int_cleanup.c_str()));
		this->port = (s_port == "" ? 6969 : atoi (s_port.c_str()));
		this->thread_count = (s_threads == "" ? 5 : atoi (s_threads.c_str())) + 1;

		cout << "port=" << this->port << endl;

		this->threads = new HANDLE[this->thread_count];

		this->isRunning = false;
		this->conn = NULL;
		this->settings = n_settings;
		this->o_settings = settings;
	}

	UDPTracker::~UDPTracker ()
	{
		int i; // loop index

		this->isRunning = false;

		// drop listener connection to continue thread loops.
		// wait for request to finish (1 second max; allot of time for a computer!).

	#ifdef linux
		close (this->sock);

		sleep (1);
	#elif defined (WIN32)
		closesocket (this->sock);

		Sleep (1000);
	#endif

		for (i = 0;i < this->thread_count;i++)
		{
	#ifdef WIN32
			TerminateThread (this->threads[i], 0x00);
	#elif defined (linux)
			pthread_detach (usi->threads[i]);
			pthread_cancel (usi->threads[i]);
	#endif
			cout << "Thread (" << ( i + 1) << "/" << ((int)this->thread_count) << ") terminated." << endl;
		}
		if (this->conn != NULL)
			db_close(this->conn);
		delete[] this->threads;
	}

	enum UDPTracker::StartStatus UDPTracker::start ()
	{
		SOCKET sock;
		SOCKADDR_IN recvAddr;
		int r,		// saves results
			i,		// loop index
			yup;	// just to set TRUE
		string dbname;// saves the Database name.

		sock = socket (AF_INET, SOCK_DGRAM, IPPROTO_UDP);
		if (sock == INVALID_SOCKET)
			return START_ESOCKET_FAILED;

	#ifdef WIN32
		recvAddr.sin_addr.S_un.S_addr = 0L;
	#elif defined (linux)
		recvAddr.sin_addr.s_addr = 0L;
	#endif
		recvAddr.sin_family = AF_INET;
		recvAddr.sin_port = m_hton16 (this->port);

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
			return START_EBIND_FAILED;
		}

		this->sock = sock;

		dbname = this->o_settings->get ("database", "file");
		if (dbname == "")
			dbname = "tracker.db";

		db_open(&this->conn, dbname.c_str());

		this->isRunning = true;
		cout << "Starting maintenance thread (1/" << ((int)this->thread_count) << ")" << endl;

		// create maintainer thread.
	#ifdef WIN32
		this->threads[0] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)_maintainance_start, (LPVOID)this, 0, NULL);
	#elif defined (linux)
		pthread_create (&usi->threads[0], NULL, _maintainance_start, usi);
	#endif

		for (i = 1;i < this->thread_count; i++)
		{
			cout << "Starting thread (" << (i + 1) << "/" << ((int)this->thread_count) << ")" << endl;
	#ifdef WIN32
			this->threads[i] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)_thread_start, (LPVOID)this, 0, NULL);
	#elif defined (linux)
			pthread_create (&(this->threads[i]), NULL, _thread_start, this);
	#endif
		}

		return START_OK;
	}

static uint64_t _get_connID (SOCKADDR_IN *remote)
{
	int base;
	uint64_t x;

	base = time(NULL);
	base /= 3600;		// changes every hour.

	x = base;
	x += remote->sin_addr.s_addr;
	return x;
}

	int UDPTracker::sendError (UDPTracker *usi, SOCKADDR_IN *remote, uint32_t transactionID, const string &msg)
	{
		struct udp_error_response error;
		int msg_sz,	// message size to send.
			i;		// copy loop
		char buff [1024];	// more than reasonable message size...

		error.action = m_hton32 (3);
		error.transaction_id = transactionID;
		error.message = (char*)msg.c_str();

		msg_sz = 4 + 4 + 1 + msg.length();

		memcpy(buff, &error, 8);
		for (i = 8;i <= msg_sz;i++)
		{
			buff[i] = msg[i - 8];
		}

		sendto(usi->sock, buff, msg_sz, 0, (SOCKADDR*)remote, sizeof(*remote));

		return 0;
	}

	int UDPTracker::handleConnection (UDPTracker *usi, SOCKADDR_IN *remote, char *data)
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

	int UDPTracker::handleAnnounce (UDPTracker *usi, SOCKADDR_IN *remote, char *data)
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
			UDPTracker::sendError (usi, remote, req->transaction_id, "Tracker doesn't allow remote IP's; Request ignored.");
			return 0;
		}

		// load peers
		q = 30;
		if (req->num_want >= 1)
			q = min (q, req->num_want);

		peers = (db_peerEntry*)malloc (sizeof(db_peerEntry) * q);

		db_load_peers(usi->conn, req->info_hash, peers, &q);

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

	int UDPTracker::handleScrape (UDPTracker *usi, SOCKADDR_IN *remote, char *data, int len)
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
			UDPTracker::sendError (usi, remote, sR->transaction_id, "Bad scrape request.");
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

			cout << "\t" << xHash << endl;

			seeders = (int32_t*)&buffer[i*12+8];
			completed = (int32_t*)&buffer[i*12+12];
			leechers = (int32_t*)&buffer[i*12+16];

			db_get_stats (usi->conn, hash, &s, &l, &c);

			*seeders = m_hton32 (s);
			*completed = m_hton32 (c);
			*leechers = m_hton32 (l);
		}
		cout.flush();

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

	int UDPTracker::resolveRequest (UDPTracker *usi, SOCKADDR_IN *remote, char *data, int r)
	{
		ConnectionRequest *cR;
		uint32_t action;

		cR = (ConnectionRequest*)data;

		action = m_hton32(cR->action);

		if ((usi->settings & UDPT_ALLOW_IANA_IP) == 0)
		{
			if (_isIANA_IP (remote->sin_addr.s_addr))
			{
				return 0;	// Access Denied: IANA reserved IP.
			}
		}

		cout << ":: " << (void*)remote->sin_addr.s_addr << ": " << remote->sin_port << " ACTION=" << action << endl;

		if (action == 0 && r >= 16)
			return UDPTracker::handleConnection (usi, remote, data);
		else if (action == 1 && r >= 98)
			return UDPTracker::handleAnnounce (usi, remote, data);
		else if (action == 2)
			return UDPTracker::handleScrape (usi, remote, data, r);
		else
		{
			cout << "E: action=" << action << ", r=" << r << endl;
			UDPTracker::sendError (usi, remote, cR->transaction_id, "Tracker couldn't understand Client's request.");
			return -1;
		}

		return 0;
	}

#ifdef WIN32
	DWORD UDPTracker::_thread_start (LPVOID arg)
#elif defined (linux)
	void* UDPTracker::_thread_start (void *arg)
#endif
	{
		UDPTracker *usi;
		SOCKADDR_IN remoteAddr;
		int addrSz,
			r;
		char tmpBuff [UDP_BUFFER_SIZE];

		usi = (UDPTracker*)arg;

		addrSz = sizeof (SOCKADDR_IN);


		while (usi->isRunning)
		{
			cout.flush();
			// peek into the first 12 bytes of data; determine if connection request or announce request.
			r = recvfrom(usi->sock, (char*)tmpBuff, UDP_BUFFER_SIZE, 0, (SOCKADDR*)&remoteAddr, &addrSz);
			if (r <= 0)
				continue;	// bad request...
			r = UDPTracker::resolveRequest (usi, &remoteAddr, tmpBuff, r);
		}

	#ifdef linux
		pthread_exit (NULL);
	#endif
		return 0;
	}

#ifdef WIN32
	DWORD UDPTracker::_maintainance_start (LPVOID arg)
#elif defined (linux)
	void* UDPTracker::_maintainance_start (void *arg)
#endif
	{
		UDPTracker *usi;

		usi = (UDPTracker *)arg;

		while (usi->isRunning)
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

};
