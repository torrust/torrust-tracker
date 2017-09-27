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
#include <fstream>
#include "tracker.hpp"
#include "logging.hpp"

namespace UDPT
{
    Tracker::Tracker() : m_logStream(nullptr)
    {
    }

    Tracker::~Tracker()
    {
        try {
            if (nullptr != m_logStream) {
                m_logStream->flush();
                delete m_logStream;
                m_logStream = nullptr;
            }
        } catch (...) {

        }
    }

    void Tracker::stop()
    {
        LOG_INFO("tracker", "Requesting components to terminate...");
        m_udpTracker->stop();

        // cause other components to destruct.
        m_apiSrv = nullptr;
        m_webApp = nullptr;
    }

    void Tracker::wait()
    {
        m_udpTracker->wait();
    }

    void Tracker::start(const boost::program_options::variables_map& conf)
    {
        setupLogging(conf);
        LOG_INFO("core", "Initializing...");

        m_udpTracker = std::shared_ptr<UDPTracker>(new UDPTracker(conf));

        if (conf["apiserver.enable"].as<bool>())
        {
            m_apiSrv = std::shared_ptr<UDPT::Server::HTTPServer>(new UDPT::Server::HTTPServer(conf));
            m_webApp = std::shared_ptr<UDPT::Server::WebApp>(new UDPT::Server::WebApp(m_apiSrv, m_udpTracker->m_conn.get(), conf));
            m_webApp->deploy();
        }

        m_udpTracker->start();
    }
    
    Tracker& Tracker::getInstance()
    {
        static Tracker s_tracker;

        return s_tracker;
    }

    boost::program_options::options_description Tracker::getConfigOptions()
    {
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
            ("logging.level", boost::program_options::value<std::string>()->default_value("warning"), "log level (fatal/error/warning/info/debug)")

#ifdef linux
            ("daemon.chdir", boost::program_options::value<std::string>()->default_value("/"), "home directory for daemon")
#endif
#ifdef WIN32 
            ("service.name", boost::program_options::value<std::string>()->default_value("udpt"), "service name to use")
#endif
            ;

        return configOptions;
    }

    void Tracker::setupLogging(const boost::program_options::variables_map& va_map) {
        Logging::Logger& logger = Logging::Logger::getLogger();

        logger.addStream(&std::cerr, Logging::Severity::FATAL);

        Logging::Severity severity;
        std::string severity_text = va_map["logging.level"].as<std::string>();
        std::transform(severity_text.begin(), severity_text.end(), severity_text.begin(), ::tolower);

        if (severity_text == "fatal") {
            severity = Logging::Severity::FATAL;
        } else if (severity_text == "error") {
            severity = Logging::Severity::ERROR;
        } else if (severity_text == "warning") {
            severity = Logging::Severity::WARNING;
        } else if (severity_text == "info") {
            severity = Logging::Severity::INFO;
        } else if (severity_text == "debug") {
            severity = Logging::Severity::DEBUG;
        } else {
            severity = Logging::Severity::UNSET;
        }

        const std::string& logFileName = va_map["logging.filename"].as<std::string>();
        Logging::Severity real_severity = (severity == Logging::Severity::UNSET ? Logging::Severity::INFO : severity);
        if (logFileName.length() == 0 || logFileName == "--") {
            logger.addStream(&std::cerr, real_severity);
        } else {
            m_logStream = new ofstream(logFileName, std::ios::app | std::ios::out);
            logger.addStream(m_logStream, real_severity);
        }

        if (severity != real_severity) {
            LOG_WARN("core", "'" << severity_text << "' is invalid, log level set to " << real_severity);
        }
    }
}
