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
#include <string>
#include <ostream>
#include <thread>
#include <atomic>
#include <condition_variable>
#include "MessageQueue.hpp"

#define LOG(severity, channel, message) \
    {\
        std::stringstream __sstream; \
        __sstream << message; \
        UDPT::Logging::Logger::getLogger().log(severity, channel, __sstream.str()); \
    }
#define LOG_INFO(channel, message) LOG(UDPT::Logging::Severity::INFO, channel, message)
#define LOG_DEBUG(channel, message) LOG(UDPT::Logging::Severity::DEBUG, channel, message)
#define LOG_WARN(channel, message) LOG(UDPT::Logging::Severity::WARNING, channel, message)
#define LOG_ERR(channel, message) LOG(UDPT::Logging::Severity::ERROR, channel, message)
#define LOG_FATAL(channel, message) LOG(UDPT::Logging::Severity::FATAL, channel, message)

namespace UDPT {
    namespace Logging {

        enum Severity {
            UNSET = 0,
            DEBUG = 10,
            INFO = 20,
            WARNING = 30,
            ERROR = 40,
            FATAL = 50
        };

        struct LogEntry {
            const std::chrono::time_point<std::chrono::system_clock> when;
            Severity severity;
            const std::string channel;
            const std::string message;
        };

        class Logger {
        public:
            static Logger& getLogger();

            void log(Severity severity, const std::string& channel, const std::string& message);

            void addStream(std::ostream *, Severity minSeverity=INFO);

        private:

            Logger();
            virtual ~Logger();
            static void worker(Logger*);
            void flush();

            std::vector<std::pair<std::ostream*, UDPT::Logging::Severity>> m_outputStreams;
            UDPT::Utils::MessageQueue<struct LogEntry> m_queue;
            std::atomic_bool m_cleaningUp;
            std::thread m_workerThread;
            std::mutex m_runningMutex;
            std::condition_variable m_runningCondition;
            Severity m_minLogLevel;
        };

    }
}
