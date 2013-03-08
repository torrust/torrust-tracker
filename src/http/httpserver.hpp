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

#ifndef HTTPSERVER_HPP_
#define HTTPSERVER_HPP_

#include <stdint.h>
#include <string>
#include <iostream>
#include <list>
#include <map>
#include "../multiplatform.h"

using namespace std;

namespace UDPT
{
	namespace API
	{
		class APIException
		{
		public:
			inline
			APIException (const string msg)
			{
				this->msg = msg;
			}

			inline
			const string& getMessage ()
			{
				return this->msg;
			}

		private:
			string msg;
		};

		class HTTPServer
		{
		public:
			class Request
			{
			public:
				enum RequestMethod {
					RM_UNKNOWN = 0,
					RM_GET = 1
				};

				Request (SOCKET sock, const SOCKADDR *sock_addr);
			private:
				friend class HTTPServer;

				enum RequestMethod requestMethod;
				string str_requestMethod;
				struct {
					unsigned int major;
					unsigned int minor;
				} httpVersion;
				SOCKET sock;
				multimap<string, string> headers;
				list<string> path;			// /some/path
				map<string, string> query;	// a=b&c=d
				const SOCKADDR *sock_addr;		// IP address+family

				void loadAndParse ();
			};

			class Response
			{
			public:
				Response (SOCKET sock);

				void sendRaw (void*, int);
				void setStatus (int code, string msg);

			private:
				friend class HTTPServer;
				SOCKET sock;
				bool isHeaderSent;
				string statusMsg;
				int statusCode;
			};

			typedef int (srvCallback) (Request *, Response *);

			HTTPServer (uint16_t port, int threads);

			void addApplication (const string path, srvCallback *callback);

			virtual ~HTTPServer ();


			static list<string> split (const string str, const string del, int limit = -1);
		private:
			typedef struct _serve_node {
				string name;	// part of path name
				map<string, struct _serve_node> children;
				srvCallback *callback;
			} serveNode;

			bool isRunning;
			serveNode rootNode;
			SOCKET sock;
			int thread_count;
			HANDLE *threads;

#ifdef WIN32
			static DWORD doServe (LPVOID arg);
#else
			static void* doServe (void* arg);
#endif

			static void handleConnection (HTTPServer *, Request *, Response *);
		};
	};
};

#endif /* HTTPSERVER_HPP_ */
