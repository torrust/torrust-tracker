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

#pragma once

#include "httpserver.hpp"
#include "../db/database.hpp"
#include "../settings.hpp"
#include <stdint.h>
#include <map>
#include <string>
using namespace std;

using namespace UDPT;
using namespace UDPT::Data;

namespace UDPT
{
	namespace Server
	{
		class WebApp
		{
		public:
			WebApp (HTTPServer *, DatabaseDriver *, Settings *);
			~WebApp ();
			void deploy ();
			

		private:
			HTTPServer *instance;
			UDPT::Data::DatabaseDriver *db;
			Settings::SettingClass *sc_api;
			std::map<std::string, list<uint32_t> > ip_whitelist;

			static void handleRoot (HTTPServer*,HTTPServer::Request*, HTTPServer::Response*);
			static void handleAnnounce (HTTPServer*,HTTPServer::Request*, HTTPServer::Response*);
			static void handleAPI (HTTPServer*,HTTPServer::Request*, HTTPServer::Response*);
			static bool isAllowedIP (WebApp *, string, uint32_t);

			void doAddTorrent (HTTPServer::Request*, HTTPServer::Response*);
			void doRemoveTorrent (HTTPServer::Request*, HTTPServer::Response*);
		};
	};
};
