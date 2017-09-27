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
#include "service.hpp"
#include <experimental/filesystem>

#ifdef WIN32 

namespace UDPT
{
    SERVICE_STATUS_HANDLE Service::s_hServiceStatus = nullptr;
    SERVICE_STATUS Service::s_serviceStatus = { 0 };

    Service::Service(const boost::program_options::variables_map& conf) : m_conf(conf)
    {

    }

    Service::~Service()
    {

    }

    void Service::install(const std::string& config_path)
    {
        std::string& binaryPath = getFilename();
        binaryPath = "\"" + binaryPath + "\" -c \"" + config_path + "\"";
        std::shared_ptr<void> svcMgr = getServiceManager(SC_MANAGER_CREATE_SERVICE);
        {
            SC_HANDLE installedService = ::CreateService(reinterpret_cast<SC_HANDLE>(svcMgr.get()),
                m_conf["service.name"].as<std::string>().c_str(),
                "UDPT Tracker",
                SC_MANAGER_CREATE_SERVICE,
                SERVICE_WIN32_OWN_PROCESS,
                SERVICE_AUTO_START,
                SERVICE_ERROR_NORMAL,
                binaryPath.c_str(),
                NULL,
                NULL,
                NULL,
                NULL,
                NULL
                );
            if (nullptr == installedService)
            {
                throw OSError();
            }

            ::CloseServiceHandle(installedService);
        }
    }

    void Service::uninstall()
    {
        std::shared_ptr<void> service = getService(DELETE);
        BOOL bRes = ::DeleteService(reinterpret_cast<SC_HANDLE>(service.get()));
        if (FALSE == bRes)
        {
            throw OSError();
        }
    }

    void Service::start()
    {
        std::shared_ptr<void> hSvc = getService(SERVICE_START);
        BOOL bRes = ::StartService(reinterpret_cast<SC_HANDLE>(hSvc.get()), 0, NULL);
        if (FALSE == bRes)
        {
            throw OSError();
        }
    }

    void Service::stop()
    {
        SERVICE_STATUS status = { 0 };

        std::shared_ptr<void> hSvc = getService(SERVICE_STOP);
        BOOL bRes = ::ControlService(reinterpret_cast<SC_HANDLE>(hSvc.get()), SERVICE_CONTROL_STOP, &status);
        if (FALSE == bRes)
        {
            throw OSError();
        }
    }

    void Service::setup()
    {
        SERVICE_TABLE_ENTRY service[] = {
            { const_cast<char*>(m_conf["service.name"].as<std::string>().c_str()), reinterpret_cast<LPSERVICE_MAIN_FUNCTION>(&Service::serviceMain) },
            {0, 0}
        };

        if (FALSE == ::StartServiceCtrlDispatcher(service))
        {
            throw OSError();
        }
    }

    DWORD Service::handler(DWORD controlCode, DWORD dwEventType, LPVOID eventData, LPVOID context)
    {
        switch (controlCode)
        {
        case SERVICE_CONTROL_INTERROGATE:
            return NO_ERROR;

        case SERVICE_CONTROL_STOP:
        {
            reportServiceStatus(SERVICE_STOP_PENDING, 0, 3000);
            Tracker::getInstance().stop();

            return NO_ERROR;
        }

        default:
            return ERROR_CALL_NOT_IMPLEMENTED;
        }
    }

    void Service::reportServiceStatus(DWORD currentState, DWORD dwExitCode, DWORD dwWaitHint)
    {
        static DWORD checkpoint = 1;

        if (currentState == SERVICE_STOPPED || currentState == SERVICE_RUNNING)
        {
            checkpoint = 0;
        }
        else
        {
            ++checkpoint;
        }

        switch (currentState)
        {
        case SERVICE_RUNNING:
            s_serviceStatus.dwControlsAccepted = SERVICE_ACCEPT_STOP;
            break;

        default:
            s_serviceStatus.dwControlsAccepted = 0;
        }

        s_serviceStatus.dwCheckPoint = checkpoint;
        s_serviceStatus.dwCurrentState = currentState;
        s_serviceStatus.dwWin32ExitCode = dwExitCode;
        s_serviceStatus.dwWaitHint = dwWaitHint;

        ::SetServiceStatus(s_hServiceStatus, &s_serviceStatus);
    }

    VOID Service::serviceMain(DWORD argc, LPCSTR argv[])
    {
        boost::log::sources::severity_channel_logger_mt<> logger(boost::log::keywords::channel = "service");

        wchar_t *commandLine = ::GetCommandLineW();
        int argCount = 0;
        std::shared_ptr<LPWSTR> args(::CommandLineToArgvW(commandLine, &argCount), ::LocalFree);
        if (nullptr == args)
        {
            BOOST_LOG_SEV(logger, boost::log::trivial::fatal) << "Failed parse command-line.";
            ::exit(-1);
        }

        if (3 != argCount)
        {
            BOOST_LOG_SEV(logger, boost::log::trivial::fatal) << "Bad command-line length (must have exactly 2 arguments).";
            ::exit(-1);
        }

        if (std::wstring(args.get()[1]) != L"-c")
        {
            BOOST_LOG_SEV(logger, boost::log::trivial::fatal) << "Argument 1 must be \"-c\".";
            ::exit(-1);
        }

        std::wstring wFilename(args.get()[2]);
        std::string cFilename(wFilename.begin(), wFilename.end());

        boost::program_options::options_description& configOptions = UDPT::Tracker::getConfigOptions();
        boost::program_options::variables_map config;
        boost::program_options::basic_parsed_options<wchar_t> parsed_options = boost::program_options::parse_config_file<wchar_t>(cFilename.c_str(), configOptions);
        boost::program_options::store(parsed_options, config);

        s_hServiceStatus = ::RegisterServiceCtrlHandlerEx(config["service.name"].as<std::string>().c_str(), Service::handler, NULL);
        if (nullptr == s_hServiceStatus)
        {
            BOOST_LOG_SEV(logger, boost::log::trivial::fatal) << "Failed to register service control handler.";
            ::exit(-1);
        }

        s_serviceStatus.dwServiceType = SERVICE_WIN32_OWN_PROCESS;
        s_serviceStatus.dwServiceSpecificExitCode = 0;

        reportServiceStatus(SERVICE_START_PENDING, 0, 0);

        {
            UDPT::Tracker& tracker = UDPT::Tracker::getInstance();
            tracker.start(config);

            reportServiceStatus(SERVICE_RUNNING, 0, 0);

            tracker.wait();

            reportServiceStatus(SERVICE_STOPPED, 0, 0);
        }
    }

    std::shared_ptr<void> Service::getService(DWORD access)
    {
        std::shared_ptr<void> serviceManager = getServiceManager(access);
        {
            SC_HANDLE service = ::OpenService(reinterpret_cast<SC_HANDLE>(serviceManager.get()), m_conf["service.name"].as<std::string>().c_str(), access);
            if (nullptr == service)
            {
                throw OSError();
            }
            return std::shared_ptr<void>(service, ::CloseServiceHandle);
        }
    }

    std::shared_ptr<void> Service::getServiceManager(DWORD access)
    {
        SC_HANDLE svcMgr = ::OpenSCManager(NULL, NULL, access);
        if (nullptr == svcMgr)
        {
            throw OSError();
        }
        return std::shared_ptr<void>(svcMgr, ::CloseServiceHandle);
    }

    std::string Service::getFilename()
    {
        char filename[MAX_PATH];
        DWORD dwRet = ::GetModuleFileName(NULL, filename, sizeof(filename) / sizeof(char));
        if (0 == dwRet)
        {
            throw OSError();
        }
        return std::string(filename);
    }
}

#endif
