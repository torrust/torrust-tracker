/*
 *	Copyright © 2012-2017 Naim A.
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

#include "multiplatform.h"
#include "udpTracker.hpp"
#include "http/httpserver.hpp"
#include "http/webapp.hpp"
#include "tracker.hpp"
#include "service.hpp"
#include "logging.hpp"

extern "C" void _signal_handler(int sig)
{
    switch (sig) {
        case SIGTERM:
        case SIGQUIT:
        case SIGINT: {
            LOG_INFO("core", "Received signal " << sig << ", requesting to stop tracker");
            UDPT::Tracker::getInstance().stop();
            break;
        }
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

#ifdef TEST
int real_main(int argc, char *argv[])
#else
int main(int argc, char *argv[])
#endif
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


    const boost::program_options::options_description& configOptions = Tracker::getConfigOptions();

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

#ifdef linux
    if (!var_map.count("interactive"))
    {
        daemonize(var_map);
    }
    ::signal(SIGTERM, _signal_handler);
    ::signal(SIGINT, _signal_handler);
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
                svc.install(var_map["config"].as<std::string>());
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
            else
            {
                std::cerr << "No such service command." << std::endl;
                return -1;
            }
        }
        catch (const UDPT::OSError& ex)
        {
            std::cerr << "An operating system error occurred: " << ex.what() << std::endl;
            return -1;
        }

        return 0;
    }

    try
    {
        svc.setup();
        return 0;
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
        std::cerr << "UDPT exception: (" << ex.getErrorCode() << "): " << ex.what();
        return -1;
    }

    LOG_INFO("core", "UDPT terminated.");

    return 0;
}
