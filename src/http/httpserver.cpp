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

#include "httpserver.hpp"
#include <cstring>
#include <cstdlib>
#include "../tools.h"

#define REQBUFFSZ	2048	// enough for all headers.

namespace UDPT
{
	namespace API
	{
		HTTPServer::HTTPServer (uint16_t port, int threads)
		{
			int r;

			this->thread_count = threads;
			this->threads = new HANDLE [threads];

			SOCKADDR_IN endpoint;
			endpoint.sin_family = AF_INET;
			endpoint.sin_port = m_hton16(port);
			endpoint.sin_addr.s_addr = 0L;

			this->sock = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
			if (this->sock == INVALID_SOCKET)
				throw APIException("Invalid Socket");

			r = bind(this->sock, (SOCKADDR*)&endpoint, sizeof(SOCKADDR_IN));
			if (r == SOCKET_ERROR)
				throw APIException("Failed to bind port.");

			this->isRunning = true;

			this->rootNode.name = "";
			this->rootNode.children.clear();
			this->rootNode.callback = NULL;

			for (int i = 0;i < threads;i++)
			{
#ifdef WIN32
				this->threads[i] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)&HTTPServer::doServe, (LPVOID)this, 0, NULL);
#else
				pthread_create (&this->threads[0], NULL, HTTPServer::doServe, (void*)this);
#endif
			}
		}

#ifdef WIN32
		DWORD HTTPServer::doServe (LPVOID arg)
#else
		void* HTTPServer::doServe (void* arg)
#endif
		{
			HTTPServer *srv = (HTTPServer*)arg;
			int r;
			SOCKADDR addr;

#ifdef linux
			socklen_t addrSz;
#else
			int addrSz;
#endif

			addrSz = sizeof (addr);
			SOCKET conn;

			while (srv->isRunning)
			{
				r = listen (srv->sock, SOMAXCONN);
				if (r == SOCKET_ERROR)
					throw APIException("Failed to listen");

				addrSz = sizeof (addr);

				conn = accept(srv->sock, &addr, &addrSz);
				if (conn == INVALID_SOCKET)
				{
					continue;
				}

				try {
					Request req = Request (conn, &addr);
					Response resp = Response (conn);

					HTTPServer::handleConnection(srv, &req, &resp);
				} catch (...) {
					cout << "ERR OCC" << endl;
				}
				closesocket(conn);
			}

#ifdef WIN32
			return 0;
#else
			return NULL;
#endif
		}

		void HTTPServer::handleConnection (HTTPServer *srv, Request *req, Response *resp)
		{
			// follow path...
			serveNode *cNode = &srv->rootNode;
			list<string>::iterator it;
			for (it = req->path.begin();(it != req->path.end() && cNode != NULL);it++)
			{
				if ((*it).length() == 0)
					continue;	// same node.

				map<string, serveNode>::iterator np;
				np = cNode->children.find((*it));
				if (np == srv->rootNode.children.end())
				{
					cNode = NULL;
					break;
				}
				else
					cNode = &np->second;
			}

			if (cNode->callback != NULL)
				cNode->callback (req, resp);
			else
			{
				// TODO: add HTTP error handler (404 NOT FOUND...)
				cout << "Page Not Found" << endl;
			}
		}

		list<string> HTTPServer::split (const string str, const string del, int limit)
		{
			list<string> lst;

			unsigned s, e;
			s = e = 0;

			while (true)
			{
				e = str.find(del, s);

				if (e == string::npos || limit - 1 == 0)
					e = str.length();

				lst.push_back(str.substr(s, e - s));

				if (e >= str.length() || limit - 1 == 0)
					break;
				s = e + del.length();
				limit--;
			}

			return lst;
		}

		void HTTPServer::addApplication (const string path, srvCallback *callback)
		{
			list<string> p = split (path, "/");
			list<string>::iterator it;

			serveNode *node = &this->rootNode;

			for (it = p.begin();it != p.end();it++)
			{
				if ((*it).length() == 0)
					continue;	// same node...

				node = &node->children[*it];
				node->name = *it;
			}
			node->callback = callback;
		}

		HTTPServer::~HTTPServer()
		{
			this->isRunning = false;
			closesocket(this->sock);
			for (int i = 0;i < this->thread_count;i++)
			{
#ifdef WIN32
				TerminateThread(this->threads[i], 0);
#else
				pthread_detach (this->threads[i]);
				pthread_cancel (this->threads[i]);
#endif
			}
			delete[] this->threads;
			cout << "ST" << endl;
		}


		HTTPServer::Request::Request(SOCKET sock, const SOCKADDR *sa)
		{
			this->sock = sock;
			this->sock_addr = sa;

			this->loadAndParse();
		}

		void HTTPServer::Request::loadAndParse ()
		{
			char buffer [REQBUFFSZ];
			int r;

			this->httpVersion.major = 0;
			this->httpVersion.minor = 0;
			this->requestMethod = RM_UNKNOWN;
			this->path.clear();
			this->headers.clear();
			this->query.clear ();
			this->str_requestMethod = "";

			r = recv (this->sock, buffer, REQBUFFSZ, 0);
			if (r <= 0)
				throw APIException("No data received from client.");

			string request = string (buffer);
			list<string> lines = HTTPServer::split(request, "\r\n");
			list<string>::iterator it, begin, end;
			begin = lines.begin();
			end = lines.end();
			for (it = begin;it != end;it++)
			{
				if (it == begin)
				{
					list<string> hLine = HTTPServer::split(*it, " ");
					if (hLine.size() < 3)
						throw APIException("Bad Request");
					this->str_requestMethod = hLine.front();
					string httpVersion = hLine.back();
					if (strncmp(httpVersion.c_str(), "HTTP/", 5) != 0)
						throw APIException("Unsupported HTTP Version");
					string vn = httpVersion.substr(5);
					this->httpVersion.major = atoi (vn.substr(0, vn.find('.')).c_str());
					this->httpVersion.minor = atoi (vn.substr(vn.find('.') + 1).c_str());

					hLine.pop_front();
					hLine.pop_back();
					string path;
					bool isF = true;
					while (!hLine.empty())
					{
						if (isF)
							isF = false;
						else
							path.append(" ");
						path.append(hLine.front());
						hLine.pop_front();
					}

					list<string> parts = HTTPServer::split(path, "?", 2);
					if (!parts.empty())
					{
						this->path = HTTPServer::split(parts.front(), "/");
						parts.pop_front();
					}
					if (!parts.empty())
					{
						string qData = parts.front();
						parts.pop_front();

						string::size_type sK, sV, eK, eV;
						sK = sV = eK = eV = 0;

						while (sK < qData.length())
						{
							eK = qData.find('=', sK);
							if (eK == string::npos) // not valid key
								break;
							sV = eK + 1;
							eV = qData.find('&', sV);
							if (eV == string::npos)
								eV = qData.length();

							this->query [qData.substr(sK, eK - sK)] = qData.substr(sV, eV - sV);

							if (eV >= qData.length())
								break;
							sK = eV + 1;
						}
					}
				}
				else
				{
					string::size_type p = (*it).find(": ");
					if (p == string::npos)
						continue;
					this->headers.insert(pair<string,string>(
							(*it).substr(0, p),
							(*it).substr(p+2)
					));
				}
			}
		}

		HTTPServer::Response::Response(SOCKET sock)
		{
			this->sock = sock;
			this->isHeaderSent = false;
			setStatus(200, "OK");
		}

		void HTTPServer::Response::setStatus (int code, string msg)
		{
			this->statusCode = code;
			this->statusMsg = msg;
		}

		void HTTPServer::Response::sendRaw (void *data, int sz)
		{
			send (this->sock, (const char*)data, sz, 0);
		}
	};
};
