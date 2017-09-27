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

#include <iostream>
#include <sstream>
#include <string>
#include <cstring>
#include <map>
#include "httpserver.hpp"
#include <boost/program_options.hpp>

using namespace std;

namespace UDPT
{
    namespace Server
    {
        /* HTTPServer */
        HTTPServer::HTTPServer (uint16_t port, int threads)
        {
            SOCKADDR_IN sa;

            memset((void*)&sa, 0, sizeof(sa));
            sa.sin_addr.s_addr = 0L;
            sa.sin_family = AF_INET;
            sa.sin_port = htons (port);

            this->init(sa, threads);
        }

        HTTPServer::HTTPServer(const boost::program_options::variables_map& conf)
        {
            list<SOCKADDR_IN> localEndpoints;
            uint16_t port;
            int threads;

            port = conf["apiserver.port"].as<unsigned short>();
            threads = conf["apiserver.threads"].as<unsigned short>();

            if (threads <= 0)
                threads = 1;

            if (localEndpoints.empty())
            {
                SOCKADDR_IN sa;
                memset((void*)&sa, 0, sizeof(sa));
                sa.sin_family = AF_INET;
                sa.sin_port = htons (port);
                sa.sin_addr.s_addr = 0L;
                localEndpoints.push_front(sa);
            }

            this->init(localEndpoints.front(), threads);
        }

        void HTTPServer::init (SOCKADDR_IN &localEndpoint, int threads)
        {
            int r;
            this->thread_count = threads;
            this->threads = new HANDLE[threads];
            this->isRunning = false;

            this->rootNode.callback = NULL;

            this->srv = ::socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
            if (this->srv == INVALID_SOCKET)
            {
                throw ServerException (1, "Failed to create Socket");
            }

            r = ::bind(this->srv, (SOCKADDR*)&localEndpoint, sizeof(localEndpoint));
            if (r == SOCKET_ERROR)
            {
                throw ServerException(2, "Failed to bind socket");
            }

            this->isRunning = true;
            for (int i = 0;i < threads;i++)
            {
#ifdef WIN32
                this->threads[i] = CreateThread (NULL, 0, (LPTHREAD_START_ROUTINE)_thread_start, this, 0, NULL);
#else
                pthread_create (&this->threads[i], NULL, &HTTPServer::_thread_start, this);
#endif
            }
        }

#ifdef WIN32
        DWORD HTTPServer::_thread_start (LPVOID arg)
#else
        void* HTTPServer::_thread_start (void *arg)
#endif
        {
            HTTPServer *s = (HTTPServer*)arg;
doSrv:
            try {
                HTTPServer::handleConnections (s);
            } catch (const ServerException &se)
            {
                cerr << "SRV ERR #" << se.getErrorCode() << ": " << se.getErrorMsg () << endl;
                goto doSrv;
            }
            return 0;
        }

        void HTTPServer::handleConnections (HTTPServer *server)
        {
            int r;
#ifdef WIN32
            int addrSz;
#else
            socklen_t addrSz;
#endif
            SOCKADDR_IN addr;
            SOCKET cli;

            while (server->isRunning)
            {
                r = ::listen(server->srv, 50);
                if (r == SOCKET_ERROR)
                {
#ifdef WIN32
                    ::Sleep(500);
#else
                    ::sleep(1);
#endif
                    continue;
                }
                addrSz = sizeof addr;
                cli = accept (server->srv, (SOCKADDR*)&addr, &addrSz);
                if (cli == INVALID_SOCKET)
                    continue;

                Response resp (cli); // doesn't throw exceptions.

                try {
                    Request req (cli, &addr);	// may throw exceptions.
                    reqCallback *cb = getRequestHandler (&server->rootNode, req.getPath());
                    if (cb == NULL)
                    {
                        // error 404
                        resp.setStatus (404, "Not Found");
                        resp.addHeader ("Content-Type", "text/html; charset=US-ASCII");
                        stringstream stream;
                        stream << "<html>";
                        stream << "<head><title>Not Found</title></head>";
                        stream << "<body><h1>Not Found</h1><div>The server couldn't find the request resource.</div><br /><hr /><div style=\"font-size:small;text-align:center;\"><a href=\"http://github.com/naim94a/udpt\">UDPT</a></div></body>";
                        stream << "</html>";
                        string str = stream.str();
                        resp.write (str.c_str(), str.length());
                    }
                    else
                    {
                        try {
                            cb (server, &req, &resp);
                        } catch (...)
                        {
                            resp.setStatus(500, "Internal Server Error");
                            resp.addHeader ("Content-Type", "text/html; charset=US-ASCII");
                            stringstream stream;
                            stream << "<html>";
                            stream << "<head><title>Internal Server Error</title></head>";
                            stream << "<body><h1>Internal Server Error</h1><div>An Error Occurred while trying to process your request.</div><br /><hr /><div style=\"font-size:small;text-align:center;\"><a href=\"http://github.com/naim94a/udpt\">UDPT</a></div></body>";
                            stream << "</html>";
                            string str = stream.str();
                            resp.write (str.c_str(), str.length());
                        }
                    }
                    resp.finalize();
                } catch (ServerException &e)
                {
                    // Error 400 Bad Request!
                }

                closesocket (cli);
            }
        }

        void HTTPServer::addApp (list<string> *path, reqCallback *cb)
        {
            list<string>::iterator it = path->begin();
            appNode *node = &this->rootNode;
            while (it != path->end())
            {
                map<string, appNode>::iterator se;
                se = node->nodes.find (*it);
                if (se == node->nodes.end())
                {
                    node->nodes[*it].callback = NULL;
                }
                node = &node->nodes[*it];
                it++;
            }
            node->callback = cb;
        }

        HTTPServer::reqCallback* HTTPServer::getRequestHandler (appNode *node, list<string> *path)
        {
            appNode *cn = node;
            list<string>::iterator it = path->begin(),
                end = path->end();
            map<string, appNode>::iterator n;
            while (true)
            {
                if (it == end)
                {
                    return cn->callback;
                }

                n = cn->nodes.find (*it);
                if (n == cn->nodes.end())
                    return NULL;	// node not found!
                cn = &n->second;

                it++;
            }
            return NULL;
        }

        void HTTPServer::setData(string k, void *d)
        {
            this->customData[k] = d;
        }

        void* HTTPServer::getData(string k)
        {
            map<string, void*>::iterator it = this->customData.find(k);
            if (it == this->customData.end())
                return NULL;
            return it->second;
        }

        HTTPServer::~HTTPServer ()
        {
            if (this->srv != INVALID_SOCKET)
                closesocket (this->srv);

            if (this->isRunning)
            {
                for (int i = 0;i < this->thread_count;i++)
                {
#ifdef WIN32
                    TerminateThread (this->threads[i], 0x00);
#else
                    pthread_detach (this->threads[i]);
                    pthread_cancel (this->threads[i]);
#endif
                }
            }

            delete[] this->threads;
        }

        /* HTTPServer::Request */
        HTTPServer::Request::Request (SOCKET cli, const SOCKADDR_IN *addr)
        {
            this->conn = cli;
            this->addr = addr;

            this->parseRequest ();
        }

        inline static char* nextReqLine (int &cPos, char *buff, int len)
        {
            for (int i = cPos;i < len - 1;i++)
            {
                if (buff[i] == '\r' && buff[i + 1] == '\n')
                {
                    buff[i] = '\0';

                    int r = cPos;
                    cPos = i + 2;
                    return (buff + r);
                }
            }

            return (buff + len);	// end
        }

        inline void parseURL (string request, list<string> *path, map<string, string> *params)
        {
            string::size_type p;
            string query, url;
            p = request.find ('?');
            if (p == string::npos)
            {
                p = request.length();
            }
            else
            {
                query = request.substr (p + 1);
            }
            url = request.substr (0, p);

            path->clear ();
            string::size_type s, e;
            s = 0;
            while (true)
            {
                e = url.find ('/', s);
                if (e == string::npos)
                    e = url.length();

                string x = url.substr (s, e - s);
                if (!(x.length() == 0 || x == "."))
                {
                    if (x == "..")
                    {
                        if (path->empty())
                            throw ServerException (1, "Hack attempt");
                        else
                            path->pop_back ();
                    }
                    path->push_back (x);
                }

                if (e == url.length())
                    break;
                s = e + 1;
            }

            string::size_type vS, vE, kS, kE;
            vS = vE = kS = kE = 0;
            while (kS < query.length())
            {
                kE = query.find ('=', kS);
                if (kE == string::npos) break;
                vS = kE + 1;
                vE = query.find ('&', vS);
                if (vE == string::npos) vE = query.length();

                params->insert (pair<string, string>( query.substr (kS, kE - kS), query.substr (vS, vE - vS) ));

                kS = vE + 1;
            }
        }

        inline void setCookies (string &data, map<string, string> *cookies)
        {
            string::size_type kS, kE, vS, vE;
            kS = 0;
            while (kS < data.length ())
            {
                kE = data.find ('=', kS);
                if (kE == string::npos)
                    break;
                vS = kE + 1;
                vE = data.find ("; ", vS);
                if (vE == string::npos)
                    vE = data.length();

                (*cookies) [data.substr (kS, kE-kS)] = data.substr (vS, vE-vS);

                kS = vE + 2;
            }
        }

        void HTTPServer::Request::parseRequest ()
        {
            char buffer [REQUEST_BUFFER_SIZE];
            int r;
            r = recv (this->conn, buffer, REQUEST_BUFFER_SIZE, 0);
            if (r == REQUEST_BUFFER_SIZE)
                throw ServerException (1, "Request Size too big.");
            if (r <= 0)
                throw ServerException (2, "Socket Error");

            char *cLine;
            int n = 0;
            int pos = 0;
            string::size_type p;
            while ( (cLine = nextReqLine (pos, buffer, r)) < (buffer + r))
            {
                string line = string (cLine);
                if (line.length() == 0) break;	// CRLF CRLF = end of headers.
                n++;

                if (n == 1)
                {
                    string::size_type uS, uE;
                    p = line.find (' ');
                    if (p == string::npos)
                        throw ServerException (5, "Malformed request method");
                    uS = p + 1;
                    this->requestMethod.str = line.substr (0, p);

                    if (this->requestMethod.str == "GET")
                        this->requestMethod.rm = RM_GET;
                    else if (this->requestMethod.str == "POST")
                        this->requestMethod.rm = RM_POST;
                    else
                        this->requestMethod.rm = RM_UNKNOWN;

                    uE = uS;
                    while (p < line.length())
                    {
                        if (p == string::npos)
                            break;
                        p = line.find (' ', p + 1);
                        if (p == string::npos)
                            break;
                        uE = p;
                    }
                    if (uE + 1 >= line.length())
                        throw ServerException (6, "Malformed request");
                    string httpVersion = line.substr (uE + 1);


                    parseURL (line.substr (uS, uE - uS), &this->path, &this->params);
                }
                else
                {
                    p = line.find (": ");
                    if (p == string::npos)
                        throw ServerException (4, "Malformed headers");
                    string key = line.substr (0, p);
                    string value = line.substr (p + 2);
                    if (key != "Cookie")
                        this->headers.insert(pair<string, string>( key, value));
                    else
                        setCookies (value, &this->cookies);
                }
            }
            if (n == 0)
                throw ServerException (3, "No Request header.");
        }

        list<string>* HTTPServer::Request::getPath ()
        {
            return &this->path;
        }

        string HTTPServer::Request::getParam (const string key)
        {
            map<string, string>::iterator it = this->params.find (key);
            if (it == this->params.end())
                return "";
            else
                return it->second;
        }

        multimap<string, string>::iterator HTTPServer::Request::getHeader (const string name)
        {
            multimap<string, string>::iterator it = this->headers.find (name);
            return it;
        }

        HTTPServer::Request::RequestMethod HTTPServer::Request::getRequestMethod ()
        {
            return this->requestMethod.rm;
        }

        string HTTPServer::Request::getRequestMethodStr ()
        {
            return this->requestMethod.str;
        }

        string HTTPServer::Request::getCookie (const string name)
        {
            map<string, string>::iterator it = this->cookies.find (name);
            if (it == this->cookies.end())
                return "";
            else
                return it->second;
        }

        const SOCKADDR_IN* HTTPServer::Request::getAddress ()
        {
            return this->addr;
        }

        /* HTTPServer::Response */
        HTTPServer::Response::Response (SOCKET cli)
        {
            this->conn = cli;

            setStatus (200, "OK");
        }

        void HTTPServer::Response::setStatus (int c, const string m)
        {
            this->status_code = c;
            this->status_msg = m;
        }

        void HTTPServer::Response::addHeader (string key, string value)
        {
            this->headers.insert (pair<string, string>(key, value));
        }

        void HTTPServer::Response::write (const char *data, int len)
        {
            if (len < 0)
                len = strlen (data);
            msg.write(data, len);
        }

        void HTTPServer::Response::finalize ()
        {
            stringstream x;
            x << "HTTP/1.1 " << this->status_code << " " << this->status_msg << "\r\n";
            multimap<string, string>::iterator it, end;
            end = this->headers.end();
            for (it = this->headers.begin(); it != end;it++)
            {
                x << it->first << ": " << it->second << "\r\n";
            }
            x << "Connection: Close\r\n";
            x << "Content-Length: " << this->msg.tellp() << "\r\n";
            x << "Server: udpt\r\n";
            x << "\r\n";
            x << this->msg.str();

            // write to socket
            send (this->conn, x.str().c_str(), x.str().length(), 0);
        }

    };
};
