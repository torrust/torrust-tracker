#pragma once

#include <memory>
#include <boost/program_options.hpp>

#include "logging.h"
#include "multiplatform.h"
#include "udpTracker.hpp"
#include "http/httpserver.hpp"
#include "http/webapp.hpp"

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

    private:
        std::shared_ptr<UDPT::UDPTracker> m_udpTracker;
        std::shared_ptr<UDPT::Server::HTTPServer> m_apiSrv;
        std::shared_ptr<UDPT::Server::WebApp> m_webApp;
        
        Tracker();
    };
}
