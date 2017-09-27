/*
 *	Copyright © 2013-2017 Naim A.
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

#pragma once

#include <stdint.h>
#include <map>
#include <string>
#include <sstream>
#include <list>
#include <boost/program_options.hpp>
#include "../multiplatform.h"
using namespace std;

#define REQUEST_BUFFER_SIZE 2048

namespace UDPT
{
    namespace Server
    {
        class ServerException
        {
        public:
            inline ServerException (int ec)
            {
                this->ec = ec;
                this->em = NULL;
            }

            inline ServerException (int ec, const char *em)
            {
                this->ec = ec;
                this->em = em;
            }

            inline const char *getErrorMsg () const
            {
                return this->em;
            }

            inline int getErrorCode () const
            {
                return this->ec;
            }
        private:
            int ec;
            const char *em;
        };

        class HTTPServer
        {
        public:
            class Request
            {
            public:
                enum RequestMethod
                {
                    RM_UNKNOWN = 0,
                    RM_GET = 1,
                    RM_POST = 2
                };

                Request (SOCKET, const SOCKADDR_IN *);
                list<string>* getPath ();

                string getParam (const string key);
                multimap<string, string>::iterator getHeader (const string name);
                RequestMethod getRequestMethod ();
                string getRequestMethodStr ();
                string getCookie (const string name);
                const SOCKADDR_IN* getAddress ();

            private:
                const SOCKADDR_IN *addr;
                SOCKET conn;
                struct {
                    int major;
                    int minor;
                } httpVer;
                struct {
                    string str;
                    RequestMethod rm;
                } requestMethod;
                list<string> path;
                map<string, string> params;
                map<string, string> cookies;
                multimap<string, string> headers;

                void parseRequest ();
            };

            class Response
            {
            public:
                Response (SOCKET conn);

                void setStatus (int, const string);
                void addHeader (string key, string value);

                int writeRaw (const char *data, int len);
                void write (const char *data, int len = -1);

            private:
                friend class HTTPServer;

                SOCKET conn;
                int status_code;
                string status_msg;
                multimap<string, string> headers;
                stringstream msg;

                void finalize ();
            };

            typedef void (reqCallback)(HTTPServer*,Request*,Response*);

            HTTPServer (uint16_t port, int threads);
            HTTPServer(const boost::program_options::variables_map& conf);

            void addApp (list<string> *path, reqCallback *);

            void setData (string, void *);
            void* getData (string);

            virtual ~HTTPServer ();

        private:
            typedef struct appNode
            {
                reqCallback *callback;
                map<string, appNode> nodes;
            } appNode;

            SOCKET srv;
            int thread_count;
            HANDLE *threads;
            bool isRunning;
            appNode rootNode;
            map<string, void*> customData;

            void init (SOCKADDR_IN &localEndpoint, int threads);

            static void handleConnections (HTTPServer *);

#ifdef WIN32
            static DWORD _thread_start (LPVOID);
#else
            static void* _thread_start (void*);
#endif

            static reqCallback* getRequestHandler (appNode *, list<string> *);
        };
    };
};
