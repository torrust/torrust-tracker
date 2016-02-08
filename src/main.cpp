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
#include <algorithm>
#include <boost/program_options.hpp>
#include <boost/date_time/posix_time/posix_time_types.hpp>
#include <boost/log/trivial.hpp>
#include <boost/log/sources/severity_channel_logger.hpp>
#include <boost/log/sinks/text_file_backend.hpp>
#include <boost/log/sinks/async_frontend.hpp>
#include <boost/log/keywords/format.hpp>
#include <boost/log/expressions.hpp>
#include <boost/log/support/date_time.hpp>
#include <boost/log/utility/setup/common_attributes.hpp>

#include "multiplatform.h"
#include "udpTracker.hpp"
#include "http/httpserver.hpp"
#include "http/webapp.hpp"
#include "tracker.hpp"
#include "service.hpp"

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

#ifdef WIN32 
void _close_wsa()
{
	::WSACleanup();
}
#endif

int main(int argc, char *argv[])
{
#ifdef WIN32
	WSADATA wsadata;
	::WSAStartup(MAKEWORD(2, 2), &wsadata);
	::atexit(_close_wsa);
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
#ifdef WIN32
		("service,s", boost::program_options::value<std::string>(), "start/stop/install/uninstall service")
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
		("apiserver.threads", boost::program_options::value<unsigned short>()->default_value(1), "threads for API server")
		("apiserver.port", boost::program_options::value<unsigned short>()->default_value(6969), "TCP port to listen on")

		("logging.filename", boost::program_options::value<std::string>()->default_value("/var/log/udpt.log"), "file to write logs to")
		("logging.level", boost::program_options::value<std::string>()->default_value("warning"), "log level (fatal/error/warning/info/debug/trace)")

#ifdef linux
		("daemon.chdir", boost::program_options::value<std::string>()->default_value("/"), "home directory for daemon")
#endif
#ifdef WIN32 
		("service.name", boost::program_options::value<std::string>()->default_value("udpt"), "service name to use")
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

	// setup logging...
	boost::log::add_common_attributes();
	boost::shared_ptr<boost::log::sinks::text_file_backend> logBackend = boost::make_shared<boost::log::sinks::text_file_backend>(
		boost::log::keywords::file_name = var_map["logging.filename"].as<std::string>(),
		boost::log::keywords::auto_flush = true,
		boost::log::keywords::open_mode = std::ios::out | std::ios::app
	);
	typedef boost::log::sinks::asynchronous_sink<boost::log::sinks::text_file_backend> udptSink_t;
	boost::shared_ptr<udptSink_t> async_sink (new udptSink_t(logBackend));
	async_sink->set_formatter(
		boost::log::expressions::stream
		<< boost::log::expressions::format_date_time<boost::posix_time::ptime>("TimeStamp", "%Y-%m-%d %H:%M:%S") << " "
		<< boost::log::expressions::attr<int>("Severity")
		<< " [" << boost::log::expressions::attr<std::string>("Channel") << "] \t"
		<< boost::log::expressions::smessage
	);
	auto loggingCore = boost::log::core::get();	
	loggingCore->add_sink(async_sink);

	boost::log::sources::severity_channel_logger_mt<> logger(boost::log::keywords::channel = "main");

	std::string severity = var_map["logging.level"].as<std::string>();
	std::transform(severity.begin(), severity.end(), severity.begin(), ::tolower);
	int severityVal = boost::log::trivial::warning;
	if ("fatal" == severity) severityVal = boost::log::trivial::fatal;
	else if ("error" == severity) severityVal = boost::log::trivial::error;
	else if ("warning" == severity) severityVal = boost::log::trivial::warning;
	else if ("info" == severity) severityVal = boost::log::trivial::info;
	else if ("debug" == severity) severityVal = boost::log::trivial::debug;
	else if ("trace" == severity) severityVal = boost::log::trivial::trace;
	else
	{
		BOOST_LOG_SEV(logger, boost::log::trivial::warning) << "Unknown debug level \"" << severity << "\" defaulting to warning";
	}

	loggingCore->set_filter(
		boost::log::trivial::severity >= severityVal
	);

#ifdef linux
	if (!var_map.count("interactive"))
	{
		daemonize(var_map);
	}
	::signal(SIGTERM, _signal_handler);
#endif
#ifdef WIN32 
	UDPT::Service svc(var_map);
	if (var_map.count("service"))
	{
		const std::string& action = var_map["service"].as<std::string>();
		try
		{
			if ("install" == action)
			{
				std::cerr << "Installing service..." << std::endl;
				svc.install();
				std::cerr << "Installed." << std::endl;
			}
			else if ("uninstall" == action)
			{
				std::cerr << "Removing service..." << std::endl;
				svc.uninstall();
				std::cerr << "Removed." << std::endl;
			}
			else if ("start" == action)
			{
				svc.start();
			}
			else if ("stop" == action)
			{
				svc.stop();
			}
		}
		catch (const UDPT::OSError& ex)
		{
			std::cerr << "An operating system error occurred: " << ex.getErrorCode() << std::endl;
			return -1;
		}

		return 0;
	}

	try 
	{
		svc.setup();
	}
	catch (const OSError& err)
	{
		if (ERROR_FAILED_SERVICE_CONTROLLER_CONNECT != err.getErrorCode())
		{
			BOOST_LOG_SEV(logger, boost::log::trivial::fatal) << "Failed to start as a Windows service: (" << err.getErrorCode() << "): " << err.what();
			return -1;
		}
	}
#endif

	try
	{
		Tracker& tracker = UDPT::Tracker::getInstance();
		tracker.start(var_map);
		tracker.wait();
	}
	catch (const UDPT::UDPTException& ex)
	{
		BOOST_LOG_SEV(logger, boost::log::trivial::fatal) << "UDPT exception: (" << ex.getErrorCode() << "): " << ex.what();
		return -1;
	}

	return 0;
}
