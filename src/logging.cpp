/*
 *	Copyright © 2012,2013 Naim A.
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

#include "logging.h"

namespace UDPT {
	Logger::Logger(Settings *s, ostream &os) : logfile (os)
	{
		this->max_time = 120;
		this->queue_limit = 50;
	}

	void Logger::log(enum LogLevel lvl, string msg)
	{
		logfile << time (NULL) << ": (" << ((char)lvl) << "): ";
		logfile << msg << "\n";
	}
};
