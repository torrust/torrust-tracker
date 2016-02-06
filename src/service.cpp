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
#include "service.hpp"

#ifdef WIN32 

namespace UDPT
{
	Service::Service(const boost::program_options::variables_map& conf) : m_conf(conf)
	{

	}

	Service::~Service()
	{

	}

	void Service::install()
	{
		std::string& binaryPath = getFilename();
		binaryPath = "\"" + binaryPath + "\"";
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

	}

	void Service::stop()
	{

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

	VOID Service::serviceMain(DWORD argc, LPCSTR argv[])
	{

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
