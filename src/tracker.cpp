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
#include "tracker.hpp"

namespace UDPT
{
	Tracker::Tracker() : m_logger(boost::log::keywords::channel = "TRACKER")
    {
		BOOST_LOG_SEV(m_logger, boost::log::trivial::debug) << "Initialized Tracker";
    }

    Tracker::~Tracker()
    {
		BOOST_LOG_SEV(m_logger, boost::log::trivial::debug) << "Destroying Tracker...";
    }

    void Tracker::stop()
	{
		BOOST_LOG_SEV(m_logger, boost::log::trivial::info) << "Requesting Tracker to terminate.";
        m_udpTracker->stop();
        wait();

        m_apiSrv = nullptr;
        m_webApp = nullptr;
        m_udpTracker = nullptr;
    }

    void Tracker::wait()
	{
		m_udpTracker->wait();
		BOOST_LOG_SEV(m_logger, boost::log::trivial::info) << "Tracker terminated.";
    }

    void Tracker::start(const boost::program_options::variables_map& conf)
	{
		BOOST_LOG_SEV(m_logger, boost::log::trivial::debug) << "Starting Tracker...";
		m_udpTracker = std::shared_ptr<UDPTracker>(new UDPTracker(conf));

        if (conf["apiserver.enable"].as<bool>())
		{
			BOOST_LOG_SEV(m_logger, boost::log::trivial::info) << "Initializing and deploying WebAPI...";
            m_apiSrv = std::shared_ptr<UDPT::Server::HTTPServer>(new UDPT::Server::HTTPServer(conf));
            m_webApp = std::shared_ptr<UDPT::Server::WebApp>(new UDPT::Server::WebApp(m_apiSrv, m_udpTracker->m_conn.get(), conf));
            m_webApp->deploy();
        }

		m_udpTracker->start();
		BOOST_LOG_SEV(m_logger, boost::log::trivial::info) << "Tracker started.";
    }
    
    Tracker& Tracker::getInstance()
    {
        static Tracker s_tracker;

        return s_tracker;
    }
}
