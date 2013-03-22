/*
 *	Copyright © 2013 Naim A.
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

		WebApp::WebApp(HTTPServer *srv, DatabaseDriver *db, Settings *settings)
		{
			this->instance = srv;
			this->db = db;
			this->sc_api = settings->getClass("api");

			Settings::SettingClass *apiKeys = settings->getClass("api.keys");
			if (apiKeys != NULL)
			{
				map<string, string>* aK = apiKeys->getMap();
				map<string, string>::iterator it, end;
				end = aK->end();
				for (it = aK->begin();it != end;it++)
				{
					string key = it->first;
					list<uint32_t> ips;

					string::size_type strp = 0;
					uint32_t ip;
					while ((ip = _getNextIPv4(strp, it->second)) != 0)
					{
						ips.push_back( m_hton32(ip) );
					}

					this->ip_whitelist.insert(pair<string, list<uint32_t> >(key, ips));
				}

			}

			srv->setData("webapp", this);
		}

		WebApp::~WebApp()
		{
		}

		void WebApp::deploy()
		{
			list<string> path;
			this->instance->addApp(&path, &WebApp::handleRoot);

			path.push_back("api");
			this->instance->addApp(&path, &WebApp::handleAPI);	// "/api"

			path.pop_back();
			path.push_back("announce");
			this->instance->addApp(&path, &WebApp::handleAnnounce);
		}

		void WebApp::handleRoot (HTTPServer *srv, HTTPServer::Request *req, HTTPServer::Response *resp)
		{
			// It would be very appreciated to keep this in the code.
			resp->write("<html>"
					"<head><title>Powered by UDPT</title></head>"
					"<body>"
					"<h2>The UDPT Project</h2>"
					"<div style=\"vertical-align:top;\">This tracker is running on UDPT Software.<br />"
					"UDPT is a open-source project, freely available for anyone to use. If you would like to obtain a copy of the software, you can get it here: <a href=\"http://code.google.com/p/udpt\">http://code.googe.com/p/udpt</a>."
					"<br /><br />If you would like to help the project grow, please donate for our hard work, effort &amp; time: <a class=\"dbtn\" href=\"https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=6735UFXPUM7BG\">"
					"<img src=\"data:image/gif;base64,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\" alt=\"Donate via PayPal\" />"
					"</a></div>"
					"<br /><hr /><div style=\"text-align:center;font-size:small;\">&copy; 2013 Naim A. | Powered by <a href=\"http://code.google.com/p/udpt\">UDPT</a></div>"
					"</body>"
					"</html>");
		}

		bool WebApp::isAllowedIP (WebApp *app, string key, uint32_t ip)
		{
			std::map<std::string, list<uint32_t> >::iterator it, end;
			end = app->ip_whitelist.end ();
			it = app->ip_whitelist.find (key);
			if (it == app->ip_whitelist.end())
				return false;	// no such key

			list<uint32_t> *lst = &it->second;
			list<uint32_t>::iterator ipit;
			for (ipit = lst->begin();ipit != lst->end();ipit++)
			{
				if (*ipit == ip)
					return true;
			}

			return false;
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

			string key = req->getParam("auth");
			if (key.length() <= 0)
				throw ServerException (0, "Bad Authentication Key");

			WebApp *app = (WebApp*)srv->getData("webapp");
			if (app == NULL)
				throw ServerException(0, "WebApp object wasn't found");

			if (!isAllowedIP(app, key, req->getAddress()->sin_addr.s_addr))
			{
				resp->setStatus(403, "Forbidden");
				resp->write("IP not whitelisted. Access Denied.");
				return;
			}

			string action = req->getParam("action");
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
