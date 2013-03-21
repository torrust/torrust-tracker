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

//					ips.push_back(0);	// end of list
//					uint32_t *rList = new uint32_t [ips.size()];
//					list<uint32_t>::iterator it;
//					int i = 0;
//					for (it = ips.begin();it != ips.end();it++)
//					{
//						rList[i++] = m_hton32((*it));
//					}
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
			path.push_back("api");
			this->instance->addApp(&path, &WebApp::handleAPI);	// "/api"
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
		}
	};
};
