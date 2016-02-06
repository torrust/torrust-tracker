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
#pragma once

#include <boost/program_options.hpp>
#include "multiplatform.h"
#include "exceptions.h"

#ifdef WIN32 
namespace UDPT
{
	class Service
	{
	public:
		Service(const boost::program_options::variables_map& conf);

		virtual ~Service();


		void install();

		void uninstall();

		void start();

		void stop();

		void setup();
	private:
		const boost::program_options::variables_map& m_conf;

		std::shared_ptr<void> getService(DWORD access);

		static VOID WINAPI handler(DWORD controlCode);

		static VOID WINAPI serviceMain(DWORD argc, LPCSTR argv[]);

		static std::shared_ptr<void> getServiceManager(DWORD access);

		static std::string getFilename();
	};
}

#endif