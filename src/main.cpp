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

#include <iostream>

#include "logging.h"
#include "multiplatform.h"
#include "udpTracker.hpp"
#include "settings.hpp"
#include "http/httpserver.hpp"
#include "http/webapp.hpp"
#include <cstdlib>	// atoi
#include <csignal>	// signal
#include <cstring>	// strlen
#include <memory.h>
#include <boost/program_options.hpp>

using namespace std;
using namespace UDPT;
using namespace UDPT::Server;

Logger *logger;
static struct {
	Settings *settings;
	UDPTracker *usi;
	WebApp *wa;
	HTTPServer *httpserver;
} Instance;


static void _print_usage ()
{
	cout << "Usage: udpt [<configuration file>]" << endl;
}

static void _doAPIStart (const boost::program_options::variables_map& settings, WebApp **wa, HTTPServer **srv, DatabaseDriver *drvr)
{
	if (!settings["apiserver.enable"].as<bool>())
	{
		return;
	}

	try 
	{
		*srv = Instance.httpserver = new HTTPServer(settings);
		*wa = Instance.wa = new WebApp(*srv, drvr, settings);
		(*wa)->deploy();
	} 
	catch (const ServerException &e)
	{
		std::cerr << "ServerException #" << e.getErrorCode() << ": " << e.getErrorMsg() << endl;
	}
}

/**
 * Sets current working directory to executables directory.
 */
static void _setCWD (char *argv0)
{
#ifdef WIN32
		wchar_t strFileName [MAX_PATH];
		DWORD r, i;
		r = GetModuleFileNameW(NULL, strFileName, MAX_PATH);
		for (i = r;i >= 0;i--)
		{
			if (strFileName[i] == '\\')
			{
				strFileName[i] = '\0';
				break;
			}
		}
		SetCurrentDirectoryW(strFileName);

#elif defined(linux)
		int len, i;
		char *strFN;
		if (argv0 != NULL)
		{
			len = strlen (argv0);
			strFN = new char [len + 1];

			for (i = len;i >= 0;i--)
			{
				if (strFN[i] == '/')
				{
					strFN = '\0';
					break;
				}
			}
			chdir (strFN);
			delete [] strFN;
		}
#endif

}

/**
 * Releases resources before exit.
 */
static void _doCleanup ()
{
	delete Instance.wa;
	delete Instance.httpserver;
	delete Instance.usi;
	delete Instance.settings;
	delete logger;

	memset (&Instance, 0, sizeof(Instance));
	logger = NULL;
}

static void _signal_handler (int sig)
{
	stringstream ss;
	ss << "Signal " << sig << " raised. Terminating...";
	logger->log(Logger::LL_INFO, ss.str());
	_doCleanup();
}

int main(int argc, char *argv[])
{
	UDPTracker *usi;
	int r;

#ifdef WIN32
	WSADATA wsadata;
	WSAStartup(MAKEWORD(2, 2), &wsadata);
#endif

	boost::program_options::options_description commandLine("Command line options");
	commandLine.add_options()
		("help,h", "produce help message")
		("all-help", "displays all help")
		("test,t", "test configuration file")
		("config,c", boost::program_options::value<std::string>()->default_value("/etc/udpt.conf"), "configuration file to use")
		;

	boost::program_options::options_description configOptions("Configuration options");
	configOptions.add_options()
		("db.driver", boost::program_options::value<std::string>()->default_value("sqlite3"), "database driver to use")
		("db.param", boost::program_options::value<std::string>()->default_value("/var/lib/udpt.db"), "database connection parameters")
		
		("tracker.is_dynamic", boost::program_options::value<bool>()->default_value(true), "Sets if the tracker is dynamic")
		("tracker.port", boost::program_options::value<unsigned short>()->default_value(6969), "UDP port to listen on")
		("tracker.threads", boost::program_options::value<unsigned>()->default_value(5), "threads to run (UDP only)")
		("tracker.allow_remotes", boost::program_options::value<bool>()->default_value(true), "allows clients to report remote IPs")
		("tracker.allow_iana_ips", boost::program_options::value<bool>()->default_value(false), "allows IANA reserved IPs to connect (useful for debugging)")
		("tracker.announce_interval", boost::program_options::value<unsigned>()->default_value(1800), "announce interval")
		("tracker.cleanup_interval", boost::program_options::value<unsigned>()->default_value(120), "sets database cleanup interval")
		
		("apiserver.enable", boost::program_options::value<bool>()->default_value(0), "Enable API server?")
		("apiserver.threads", boost::program_options::value<unsigned>()->default_value(1), "threads for API server")
		("apiserver.port", boost::program_options::value<unsigned short>()->default_value(6969), "TCP port to listen on")

		("logging.filename", boost::program_options::value<std::string>()->default_value("stdout"), "file to write logs to")
		("logging.level", boost::program_options::value<std::string>()->default_value("warning"), "log level (error/warning/info/debug)")
		;

	boost::program_options::variables_map var_map;
	boost::program_options::store(boost::program_options::parse_command_line(argc, argv, commandLine), var_map);
	boost::program_options::notify(var_map);

	if (var_map.count("help"))
	{
		std::cout << "UDP Tracker (UDPT) " << VERSION << " (" << PLATFORM << ")" << std::endl
			<< "Copyright 2012-2016 Naim A. <naim94a@gmail.com>" << std::endl
			<< "Build Date: " << __DATE__ << std::endl << std::endl;
		
		std::cout << commandLine << std::endl;
		return 0;
	}

	if (var_map.count("all-help"))
	{
		std::cout << commandLine << std::endl;
		std::cout << configOptions << std::endl;
		return 0;
	}

	std::string config_filename(var_map["config"].as<std::string>());
	bool isTest = var_map.count("test");

	if (var_map.count("config"))
	{
		try
		{
			boost::program_options::basic_parsed_options<wchar_t> parsed_options = boost::program_options::parse_config_file<wchar_t>(config_filename.c_str(), configOptions);
			boost::program_options::store(
				parsed_options,
				var_map);
		}
		catch (const boost::program_options::error& ex)
		{
			std::cerr << "ERROR: " << ex.what() << std::endl;
			return -1;
		}

		if (isTest)
		{
			std::cout << "Config OK" << std::endl;
			return 0;
		}
	}

	memset(&Instance, 0, sizeof(Instance));

#ifdef SIGBREAK
	signal(SIGBREAK, &_signal_handler);
#endif
#ifdef SIGTERM
	signal(SIGTERM, &_signal_handler);
#endif
#ifdef SIGABRT
	signal(SIGABRT, &_signal_handler);
#endif
#ifdef SIGINT
	signal(SIGINT, &_signal_handler);
#endif
	
	try
	{
		logger = new Logger(var_map);
	}
	catch (const std::exception& ex)
	{
		std::cerr << "Failed to initialize logger: " << ex.what() << std::endl;
		return -1;
	}

	usi = Instance.usi = new UDPTracker(var_map);

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

	_doAPIStart(var_map, &wa, &apiSrv, usi->conn);

	cout << "Hit Control-C to exit." << endl;

	usi->wait();

cleanup:
	cout << endl << "Goodbye." << endl;

#ifdef WIN32
	WSACleanup();
#endif

	return 0;
}
