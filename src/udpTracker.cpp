/*
 *	Copyright © 2012-2016 Naim A.
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

#include <cstdlib> // atoi
#include <cstring>
#include <ctime>
#include <iostream>
#include <sstream>
#include <list>
#include "udpTracker.hpp"
#include "tools.h"
#include "multiplatform.h"
#include "logging.h"

extern UDPT::Logger *logger;

using namespace UDPT::Data;

#define UDP_BUFFER_SIZE		2048

namespace UDPT
{
	UDPTracker::UDPTracker(const boost::program_options::variables_map& conf) : m_conf(conf)
	{

		this->m_allowRemotes = conf["tracker.allow_remotes"].as<bool>();
		this->m_allowIANA_IPs = conf["tracker.allow_iana_ips"].as<bool>();
		this->m_isDynamic = conf["tracker.is_dynamic"].as<bool>();

		this->m_announceInterval = conf["tracker.announce_interval"].as<unsigned>();
		this->m_cleanupInterval = conf["tracker.cleanup_interval"].as<unsigned>();
		this->m_port = conf["tracker.port"].as<unsigned short>();
		this->m_threadCount = conf["tracker.threads"].as<unsigned>() + 1;

		std::list<SOCKADDR_IN> addrs;

		if (addrs.empty())
		{
			SOCKADDR_IN sa;
			sa.sin_port = m_hton16(m_port);
			sa.sin_addr.s_addr = 0L;
			addrs.push_back(sa);
		}

		this->m_localEndpoint = addrs.front();


		this->m_threads = new HANDLE[this->m_threadCount];

		this->m_isRunning = false;
		this->conn = nullptr;
	}

	UDPTracker::~UDPTracker()
	{
		int i; // loop index

		this->m_isRunning = false;

		// drop listener connection to continue thread loops.
		// wait for request to finish (1 second max; allot of time for a computer!).

	#ifdef linux
		::close(this->m_sock);

		::sleep(1);
	#elif defined (WIN32)
		::closesocket(this->m_sock);

		::Sleep(1000);
	#endif

		for (i = 0;i < this->m_threadCount;i++)
		{
	#ifdef WIN32
			::TerminateThread(this->m_threads[i], 0x00);
	#elif defined (linux)
			::pthread_detach(this->m_threads[i]);
			::pthread_cancel(this->m_threads[i]);
	#endif
			std::stringstream str;
			str << "Thread (" << (i + 1) << "/" << ((int)this->m_threadCount) << ") terminated.";
			logger->log(Logger::LL_INFO, str.str());
		}
		if (this->conn != NULL)
			delete this->conn;
		delete[] this->m_threads;
	}

	void UDPTracker::wait()
	{
#ifdef WIN32
		::WaitForMultipleObjects(this->m_threadCount, this->m_threads, TRUE, INFINITE);
#else
		int i;
		for (i = 0;i < this->m_threadCount; i++)
		{
			::pthread_join(this->m_threads[i], NULL);
		}
#endif
	}

	void UDPTracker::start()
	{
		std::stringstream ss;
		SOCKET sock;
		int r,		// saves results
			i,		// loop index
			yup;	// just to set TRUE
		std::string dbname;// saves the Database name.

		sock = ::socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
		if (sock == INVALID_SOCKET)
		{
			throw UDPT::UDPTException("Failed to create socket");
		}

		yup = 1;
		::setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, (const char*)&yup, 1);

		this->m_localEndpoint.sin_family = AF_INET;
		r = ::bind(sock, reinterpret_cast<SOCKADDR*>(&this->m_localEndpoint), sizeof(SOCKADDR_IN));

		if (r == SOCKET_ERROR)
		{
	#ifdef WIN32
			::closesocket(sock);
	#elif defined (linux)
			::close(sock);
	#endif
			throw UDPT::UDPTException("Failed to bind socket.");
		}

		this->m_sock = sock;

		this->conn = new Data::SQLite3Driver(m_conf, this->m_isDynamic);

		this->m_isRunning = true;

		ss.str("");
		ss << "Starting maintenance thread (1/" << ((int)this->m_threadCount) << ")";
		logger->log(Logger::LL_INFO, ss.str());

		// create maintainer thread.
	#ifdef WIN32
		this->m_threads[0] = ::CreateThread(NULL, 0, reinterpret_cast<LPTHREAD_START_ROUTINE>(_maintainance_start), (LPVOID)this, 0, NULL);
	#elif defined (linux)
		::pthread_create(&this->m_threads[0], NULL, _maintainance_start, (void*)this);
	#endif

		for (i = 1;i < this->m_threadCount; i++)
		{
			ss.str("");
			ss << "Starting thread (" << (i + 1) << "/" << ((int)this->m_threadCount) << ")";
			logger->log(Logger::LL_INFO, ss.str());

			#ifdef WIN32
			this->m_threads[i] = ::CreateThread(NULL, 0, reinterpret_cast<LPTHREAD_START_ROUTINE>(_thread_start), (LPVOID)this, 0, NULL);
	#elif defined (linux)
			::pthread_create(&(this->m_threads[i]), NULL, _thread_start, (void*)this);
	#endif
		}
	}

	int UDPTracker::sendError(UDPTracker* usi, SOCKADDR_IN* remote, uint32_t transactionID, const std::string &msg)
	{
		struct udp_error_response error;
		int msg_sz,	// message size to send.
			i;		// copy loop
		char buff [1024];	// more than reasonable message size...

		error.action = m_hton32 (3);
		error.transaction_id = transactionID;
		error.message = (char*)msg.c_str();

		msg_sz = 4 + 4 + 1 + msg.length();

		// test against overflow message. resolves issue 4.
		if (msg_sz > 1024)
			return -1;

		::memcpy(buff, &error, 8);
		for (i = 8;i <= msg_sz;i++)
		{
			buff[i] = msg[i - 8];
		}

		::sendto(usi->m_sock, buff, msg_sz, 0, reinterpret_cast<SOCKADDR*>(remote), sizeof(*remote));

		return 0;
	}

	int UDPTracker::handleConnection(UDPTracker *usi, SOCKADDR_IN *remote, char *data)
	{
		ConnectionRequest *req = reinterpret_cast<ConnectionRequest*>(data);
		ConnectionResponse resp;

		resp.action = m_hton32(0);
		resp.transaction_id = req->transaction_id;

		if (!usi->conn->genConnectionId(&resp.connection_id,
				m_hton32(remote->sin_addr.s_addr),
				m_hton16(remote->sin_port)))
		{
			return 1;
		}

		::sendto(usi->m_sock, (char*)&resp, sizeof(ConnectionResponse), 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

		return 0;
	}

	int UDPTracker::handleAnnounce (UDPTracker *usi, SOCKADDR_IN *remote, char *data)
	{
		AnnounceRequest *req;
		AnnounceResponse *resp;
		int q,		// peer counts
			bSize,	// message size
			i;		// loop index
		DatabaseDriver::PeerEntry *peers;
		DatabaseDriver::TorrentEntry tE;

		uint8_t buff [1028];	// Reasonable buffer size. (header+168 peers)

		req = (AnnounceRequest*)data;

		if (!usi->conn->verifyConnectionId(req->connection_id,
				m_hton32(remote->sin_addr.s_addr),
				m_hton16(remote->sin_port)))
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

		if (!usi->m_allowRemotes && req->ip_address != 0)
		{
			UDPTracker::sendError(usi, remote, req->transaction_id, "Tracker doesn't allow remote IP's; Request ignored.");
			return 0;
		}

		if (!usi->conn->isTorrentAllowed(req->info_hash))
		{
			UDPTracker::sendError(usi, remote, req->transaction_id, "info_hash not registered.");
			return 0;
		}

		// load peers
		q = 30;
		if (req->num_want >= 1)
			q = min (q, req->num_want);

		peers = new DatabaseDriver::PeerEntry [q];


		DatabaseDriver::TrackerEvents event;
		switch (req->event)
		{
		case 1:
			event = DatabaseDriver::EVENT_COMPLETE;
			break;
		case 2:
			event = DatabaseDriver::EVENT_START;
			break;
		case 3:
			event = DatabaseDriver::EVENT_STOP;
			break;
		default:
			event = DatabaseDriver::EVENT_UNSPEC;
			break;
		}

		if (event == DatabaseDriver::EVENT_STOP)
			q = 0;	// no need for peers when stopping.

		if (q > 0)
			usi->conn->getPeers(req->info_hash, &q, peers);

		bSize = 20; // header is 20 bytes
		bSize += (6 * q); // + 6 bytes per peer.

		tE.info_hash = req->info_hash;
		usi->conn->getTorrentInfo(&tE);

		resp = (AnnounceResponse*)buff;
		resp->action = m_hton32(1);
		resp->interval = m_hton32 ( usi->m_announceInterval );
		resp->leechers = m_hton32(tE.leechers);
		resp->seeders = m_hton32 (tE.seeders);
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
		delete[] peers;
		::sendto(usi->m_sock, (char*)buff, bSize, 0, (SOCKADDR*)remote, sizeof(SOCKADDR_IN));

		// update DB.
		uint32_t ip;
		if (req->ip_address == 0) // default
			ip = m_hton32 (remote->sin_addr.s_addr);
		else
			ip = req->ip_address;
		usi->conn->updatePeer(req->peer_id, req->info_hash, ip, req->port,
				req->downloaded, req->left, req->uploaded, event);

		return 0;
	}

	int UDPTracker::handleScrape(UDPTracker *usi, SOCKADDR_IN *remote, char *data, int len)
	{
		ScrapeRequest *sR = reinterpret_cast<ScrapeRequest*>(data);
		int v,	// validation helper
			c,	// torrent counter
			i,	// loop counter
			j;	// loop counter
		uint8_t hash [20];
		ScrapeResponse *resp;
		uint8_t buffer [1024];	// up to 74 torrents can be scraped at once (17*74+8) < 1024

		// validate request length:
		v = len - 16;
		if (v < 0 || v % 20 != 0)
		{
			UDPTracker::sendError(usi, remote, sR->transaction_id, "Bad scrape request.");
			return 0;
		}

		if (!usi->conn->verifyConnectionId(sR->connection_id,
				m_hton32(remote->sin_addr.s_addr),
				m_hton16(remote->sin_port)))
		{
			return 1;
		}

		// get torrent count.
		c = v / 20;

		resp = reinterpret_cast<ScrapeResponse*>(buffer);
		resp->action = m_hton32(2);
		resp->transaction_id = sR->transaction_id;

		for (i = 0;i < c;i++)
		{
			int32_t *seeders,
				*completed,
				*leechers;

			for (j = 0; j < 20;j++)
				hash[j] = data[j + (i*20)+16];

			seeders = (int32_t*)&buffer[i*12+8];
			completed = (int32_t*)&buffer[i*12+12];
			leechers = (int32_t*)&buffer[i*12+16];

			DatabaseDriver::TorrentEntry tE;
			tE.info_hash = hash;
			if (!usi->conn->getTorrentInfo(&tE))
			{
				sendError(usi, remote, sR->transaction_id, "Scrape Failed: couldn't retrieve torrent data");
				return 0;
			}

			*seeders = m_hton32(tE.seeders);
			*completed = m_hton32(tE.completed);
			*leechers = m_hton32(tE.leechers);
		}

		::sendto(usi->m_sock, reinterpret_cast<const char*>(buffer), sizeof(buffer), 0, reinterpret_cast<SOCKADDR*>(remote), sizeof(SOCKADDR_IN));

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
		ConnectionRequest* cR = reinterpret_cast<ConnectionRequest*>(data);
		uint32_t action;

		action = m_hton32(cR->action);

		if (!usi->m_allowIANA_IPs)
		{
			if (_isIANA_IP(remote->sin_addr.s_addr))
			{
				return 0;	// Access Denied: IANA reserved IP.
			}
		}

		if (action == 0 && r >= 16)
			return UDPTracker::handleConnection(usi, remote, data);
		else if (action == 1 && r >= 98)
			return UDPTracker::handleAnnounce(usi, remote, data);
		else if (action == 2)
			return UDPTracker::handleScrape(usi, remote, data, r);
		else
		{
			UDPTracker::sendError(usi, remote, cR->transaction_id, "Tracker couldn't understand Client's request.");
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

#ifdef linux
		socklen_t addrSz;
#else
		int addrSz;
#endif

		int r;
		char tmpBuff[UDP_BUFFER_SIZE];

		usi = reinterpret_cast<UDPTracker*>(arg);

		addrSz = sizeof(SOCKADDR_IN);


		while (usi->m_isRunning)
		{
			// peek into the first 12 bytes of data; determine if connection request or announce request.
			r = ::recvfrom(usi->m_sock, (char*)tmpBuff, UDP_BUFFER_SIZE, 0, (SOCKADDR*)&remoteAddr, &addrSz);
			if (r <= 0)
				continue;	// bad request...
			r = UDPTracker::resolveRequest(usi, &remoteAddr, tmpBuff, r);
		}

	#ifdef linux
		::pthread_exit (NULL);
	#endif
		return 0;
	}

#ifdef WIN32
	DWORD UDPTracker::_maintainance_start(LPVOID arg)
#elif defined (linux)
	void* UDPTracker::_maintainance_start(void *arg)
#endif
	{
		UDPTracker* usi = reinterpret_cast<UDPTracker*>(arg);

		while (usi->m_isRunning)
		{
			usi->conn->cleanup();

#ifdef WIN32
			::Sleep(usi->m_cleanupInterval * 1000);
#elif defined (linux)
			::sleep(usi->m_cleanupInterval);
#else
#error Unsupported OS.
#endif
		}

		return 0;
	}

};
