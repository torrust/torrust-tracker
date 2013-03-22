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

#include <iostream>

#include "multiplatform.h"
#include "udpTracker.hpp"
#include "settings.hpp"
#include "http/httpserver.hpp"
#include "http/webapp.hpp"
#include <cstdlib>	// atoi

using namespace std;
using namespace UDPT;
using namespace UDPT::Server;

static void _print_usage ()
{
	cout << "Usage: udpt [<configuration file>]" << endl;
}

static void _doAPIStart (Settings *settings, WebApp **wa, HTTPServer **srv, DatabaseDriver *drvr)
{
	if (settings == NULL)
		return;
	Settings::SettingClass *sc = settings->getClass("apiserver");
	if (sc == NULL)
		return;		// no settings set!

	if (sc->get("enable") != "1")
	{
		cerr << "API Server not enabled." << endl;
		return;
	}

	string s_port = sc->get("port");
	string s_threads = sc->get("threads");

	uint16_t port = (s_port == "" ? 6969 : atoi (s_port.c_str()));
	uint16_t threads = (s_threads == "" ? 1 : atoi (s_threads.c_str()));

	if (threads <= 0)
		threads = 1;

	try {
		*srv = new HTTPServer (port, threads);
		*wa = new WebApp (*srv, drvr, settings);
		(*wa)->deploy();
	} catch (ServerException &e)
	{
		cerr << "ServerException #" << e.getErrorCode() << ": " << e.getErrorMsg() << endl;
	}
}

int main(int argc, char *argv[])
{
	Settings *settings = NULL;
	UDPTracker *usi = NULL;
	string config_file;
	int r;

#ifdef WIN32
	WSADATA wsadata;
	WSAStartup(MAKEWORD(2, 2), &wsadata);
#endif

	cout << "UDP Tracker (UDPT) " << VERSION << endl;
	cout << "Copyright 2012,2013 Naim Abda <naim94a@gmail.com>\n\tReleased under the GPLv3 License." << endl;
	cout << "Build Date: " << __DATE__ << endl << endl;

	config_file = "udpt.conf";

	if (argc <= 1)
	{
		_print_usage ();
	}

	settings = new Settings (config_file);

	if (!settings->load())
	{
		const char strDATABASE[] = "database";
		const char strTRACKER[] = "tracker";
		const char strAPISRV [] = "apiserver";
		// set default settings:

		settings->set (strDATABASE, "driver", "sqlite3");
		settings->set (strDATABASE, "file", "tracker.db");

		settings->set (strTRACKER, "port", "6969");		// UDP PORT
		settings->set (strTRACKER, "threads", "5");
		settings->set (strTRACKER, "allow_remotes", "yes");
		settings->set (strTRACKER, "allow_iana_ips", "yes");
		settings->set (strTRACKER, "announce_interval", "1800");
		settings->set (strTRACKER, "cleanup_interval", "120");

		settings->set (strAPISRV, "enable", "1");
		settings->set (strAPISRV, "threads", "1");
		settings->set (strAPISRV, "port", "6969");	// TCP PORT

		settings->save();
		cerr << "Failed to read from '" << config_file.c_str() << "'. Using default settings." << endl;
	}

	usi = new UDPTracker (settings);

	HTTPServer *apiSrv = NULL;
	WebApp *wa = NULL;

	r = usi->start();
	if (r != UDPTracker::START_OK)
	{
		cerr << "Error While trying to start server." << endl;
		switch (r)
		{
		case UDPTracker::START_ESOCKET_FAILED:
			cerr << "Failed to create socket." << endl;
			break;
		case UDPTracker::START_EBIND_FAILED:
			cerr << "Failed to bind socket." << endl;
			break;
		default:
			cerr << "Unknown Error" << endl;
			break;
		}
		goto cleanup;
	}

	_doAPIStart(settings, &wa, &apiSrv, usi->conn);

	cout << "Press Any key to exit." << endl;

	cin.get();

cleanup:
	cout << endl << "Goodbye." << endl;

	delete usi;
	delete settings;
	delete apiSrv;
	delete wa;

#ifdef WIN32
	WSACleanup();
#endif

	return 0;
}
