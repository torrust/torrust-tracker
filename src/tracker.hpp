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
#pragma once

#include <memory>
#include <boost/program_options.hpp>

#include <boost/date_time/posix_time/posix_time_types.hpp>

#include "udpTracker.hpp"
#include "WebApp.hpp"

namespace UDPT
{
    class Tracker
    {
    public:

        virtual ~Tracker();

        void stop();

        void start(const boost::program_options::variables_map& conf);

        void wait();

        static Tracker& getInstance();

        static boost::program_options::options_description getConfigOptions();

    private:
        std::shared_ptr<UDPT::UDPTracker> m_udpTracker;
        std::shared_ptr<UDPT::WebApp> m_webApp;

        Tracker();

        void setupLogging(const boost::program_options::variables_map& va_map);
        std::ostream *m_logStream;
    };
}
