#include "tracker.hpp"

namespace UDPT
{
    Tracker::Tracker()
    {

    }

    Tracker::~Tracker()
    {

    }

    void Tracker::stop()
    {
        m_udpTracker->stop();
        wait();

        m_apiSrv = nullptr;
        m_webApp = nullptr;
        m_udpTracker = nullptr;
    }

    void Tracker::wait()
    {
        m_udpTracker->wait();
    }

    void Tracker::start(const boost::program_options::variables_map& conf)
    {
        m_udpTracker = std::shared_ptr<UDPTracker>(new UDPTracker(conf));

        if (conf["apiserver.enable"].as<bool>())
        {
            m_apiSrv = std::shared_ptr<UDPT::Server::HTTPServer>(new UDPT::Server::HTTPServer(conf));
            m_webApp = std::shared_ptr<UDPT::Server::WebApp>(new UDPT::Server::WebApp(m_apiSrv, m_udpTracker->conn, conf));
            m_webApp->deploy();
        }

        m_udpTracker->start();
    }
    
    Tracker& Tracker::getInstance()
    {
        static Tracker s_tracker;

        return s_tracker;
    }
}
