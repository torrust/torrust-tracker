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

#include <cstdlib>	// atoi
#include <csignal>	// signal
#include <cstring>	// strlen
#include <memory>
#include <boost/program_options.hpp>

#include "logging.h"
#include "multiplatform.h"
#include "udpTracker.hpp"
#include "http/httpserver.hpp"
#include "http/webapp.hpp"
#include "tracker.hpp"

UDPT::Logger *logger = NULL;

static void _signal_handler(int sig)
{
	switch (sig)
	{
		case SIGTERM:
			UDPT::Tracker::getInstance().stop();
			break;
	}
}

#ifdef linux
static void daemonize(const boost::program_options::variables_map& conf)
{
	if (1 == ::getppid()) return; // already a daemon
	int r = ::fork();
	if (0 > r) ::exit(-1); // failed to daemonize.
	if (0 < r) ::exit(0); // parent exists.

	::umask(0);
	::setsid();

	// close all fds.
	for (int i = ::getdtablesize(); i >=0; --i)
	{
		::close(i);
	}

	::chdir(conf["daemon.chdir"].as<std::string>().c_str());

}
#endif

int main(int argc, char *argv[])
{
	Tracker& tracker = UDPT::Tracker::getInstance();

#ifdef WIN32
	WSADATA wsadata;
	::WSAStartup(MAKEWORD(2, 2), &wsadata);
#endif

	boost::program_options::options_description commandLine("Command line options");
	commandLine.add_options()
		("help,h", "produce help message")
		("all-help", "displays all help")
		("test,t", "test configuration file")
		("config,c", boost::program_options::value<std::string>()->default_value("/etc/udpt.conf"), "configuration file to use")
#ifdef linux
		("interactive,i", "doesn't start as daemon")
#endif
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

#ifdef linux
		("daemon.chdir", boost::program_options::value<std::string>()->default_value("/"), "home directory for daemon")
#endif
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
	bool isTest = (0 != var_map.count("test"));

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

	std::shared_ptr<UDPT::Logger> spLogger;
	try
	{
		spLogger = std::shared_ptr<UDPT::Logger>(new UDPT::Logger(var_map));
		logger = spLogger.get();
	}
	catch (const std::exception& ex)
	{
		std::cerr << "Failed to initialize logger: " << ex.what() << std::endl;
		return -1;
	}

#ifdef linux
	if (!var_map.count("interactive"))
	{
		daemonize(var_map);
	}
	::signal(SIGTERM, _signal_handler);
#endif

	tracker.start(var_map);
	tracker.wait();

#ifdef WIN32
	::WSACleanup();
#endif

	return 0;
}
