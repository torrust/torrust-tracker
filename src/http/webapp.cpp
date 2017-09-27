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

#include "webapp.hpp"
#include "../tools.h"
#include <iostream>
#include <cstdio>
using namespace std;

namespace UDPT
{
    namespace Server
    {

        static uint32_t _getNextIPv4 (string::size_type &i, string &line)
        {
            string::size_type len = line.length();
            char c;
            while (i < len)
            {
                c = line.at(i);
                if (c >= '0' && c <= '9')
                    break;
                i++;
            }

            uint32_t ip = 0;
            for (int n = 0;n < 4;n++)
            {
                int cn = 0;
                while (i < len)
                {
                    c = line.at (i++);
                    if (c == '.' || ((c == ' ' || c == ',' || c == ';') && n == 3))
                        break;
                    else if (!(c >= '0' && c <= '9'))
                        return 0;
                    cn *= 10;
                    cn += (c - '0');
                }
                ip *= 256;
                ip += cn;
            }
            return ip;
        }

        static bool _hex2bin (uint8_t *data, const string str)
        {
            int len = str.length();

            if (len % 2 != 0)
                return false;

            char a, b;
            uint8_t c;
            for (int i = 0;i < len;i+=2)
            {
                a = str.at (i);
                b = str.at (i + 1);
                c = 0;

                if (a >= 'a' && a <= 'f')
                    a = (a - 'a') + 10;
                else if (a >= '0' && a <= '9')
                    a = (a - '0');
                else
                    return false;

                if (b >= 'a' && b <= 'f')
                    b = (b - 'a') + 10;
                else if (b >= '0' && b <= '9')
                    b = (b - '0');
                else
                    return false;

                c = (a * 16) + b;

                data [i / 2] = c;
            }

            return true;
        }

        WebApp::WebApp(std::shared_ptr<HTTPServer> srv, DatabaseDriver *db, const boost::program_options::variables_map& conf) : m_conf(conf), m_server(srv)
        {
            this->db = db;
            // TODO: Implement authentication by keys

            m_server->setData("webapp", this);
        }

        WebApp::~WebApp()
        {
        }

        void WebApp::deploy()
        {
            list<string> path;
            m_server->addApp(&path, &WebApp::handleRoot);

            path.push_back("api");
            m_server->addApp(&path, &WebApp::handleAPI);	// "/api"

            path.pop_back();
            path.push_back("announce");
            m_server->addApp(&path, &WebApp::handleAnnounce);
        }

        void WebApp::handleRoot(HTTPServer *srv, HTTPServer::Request *req, HTTPServer::Response *resp)
        {
            // It would be very appreciated to keep this in the code.
            resp->write("<html>"
                    "<head><title>UDPT Torrent Tracker</title></head>"
                    "<body>"
                    "<div style=\"vertical-align:top;\">This tracker is running on UDPT Software.</div>"
                    "<br /><hr /><div style=\"text-align:center;font-size:small;\"><a href=\"http://github.com/naim94a/udpt\">UDPT</a></div>"
                    "</body>"
                    "</html>");
        }

        void WebApp::doRemoveTorrent (HTTPServer::Request *req, HTTPServer::Response *resp)
        {
            string strHash = req->getParam("hash");
            if (strHash.length() != 40)
            {
                resp->write("{\"error\":\"Hash length must be 40 characters.\"}");
                return;
            }
            uint8_t hash [20];
            if (!_hex2bin(hash, strHash))
            {
                resp->write("{\"error\":\"invalid info_hash.\"}");
                return;
            }


            if (this->db->removeTorrent(hash))
                resp->write("{\"success\":true}");
            else
                resp->write("{\"error\":\"failed to remove torrent from DB\"}");
        }

        void WebApp::doAddTorrent (HTTPServer::Request *req, HTTPServer::Response *resp)
        {
            std::string strHash = req->getParam("hash");
            if (strHash.length() != 40)
            {
                resp->write("{\"error\":\"Hash length must be 40 characters.\"}");
                return;
            }
            uint8_t hash [20];
            if (!_hex2bin(hash, strHash))
            {
                resp->write("{\"error\":\"invalid info_hash.\"}");
                return;
            }

            if (this->db->addTorrent(hash))
                resp->write("{\"success\":true}");
            else
                resp->write("{\"error\":\"failed to add torrent to DB\"}");
        }

        void WebApp::handleAnnounce (HTTPServer *srv, HTTPServer::Request *req, HTTPServer::Response *resp)
        {
            resp->write("d14:failure reason42:this is a UDP tracker, not a HTTP tracker.e");
        }

        void WebApp::handleAPI(HTTPServer *srv, HTTPServer::Request *req, HTTPServer::Response *resp)
        {
            if (req->getAddress()->sin_family != AF_INET)
            {
                throw ServerException (0, "IPv4 supported Only.");
            }

            WebApp *app = (WebApp*)srv->getData("webapp");
            if (app == NULL)
                throw ServerException(0, "WebApp object wasn't found");

            if (req->getAddress()->sin_addr.s_addr != 0x0100007f)
            {
                resp->setStatus(403, "Forbidden");
                resp->write("Access Denied. Only 127.0.0.1 can access this method.");
                return;
            }

            std::string action = req->getParam("action");
            if (action == "add")
                app->doAddTorrent(req, resp);
            else if (action == "remove")
                app->doRemoveTorrent(req, resp);
            else
            {
                resp->write("{\"error\":\"unknown action\"}");
            }
        }
    };
};
