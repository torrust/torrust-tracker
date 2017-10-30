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
#include "logging.hpp"
#include <sstream>
#include <chrono>
#include <cstring>
#include <iostream>

namespace UDPT {
    namespace Logging {

        Logger::Logger() : m_cleaningUp(false), m_workerThread(Logger::worker, this), m_minLogLevel(Severity::FATAL) {
        }

        Logger::~Logger() {
            try {
                // prevent new log messages from entering queue.
                m_cleaningUp = true;

                // tell thread to exit
                m_runningCondition.notify_one();

                // wait for worker to terminate
                m_workerThread.join();
                // flush any remaining logs
                flush();

                // flush iostreams
                for (std::vector<std::pair<std::ostream*, Severity>>::const_iterator it = m_outputStreams.begin(); it != m_outputStreams.end(); ++it) {
                    it->first->flush();
                }
            } catch (...) {
                // can't do much here... this is a logging class...
            }
        }

        Logger& Logger::getLogger() {
            static Logger instance;

            return instance;
        }

        void Logger::log(Severity severity, const std::string& channel, const std::string& message) {
            if (severity < m_minLogLevel) {
                return;
            }

            m_queue.Push(LogEntry{
                    .when=std::chrono::system_clock::now(),
                    .severity=severity,
                    .channel=channel,
                    .message=message
            });
        }

        void Logger::addStream(std::ostream *stream, Severity severity) {
            m_minLogLevel = m_minLogLevel > severity ? severity : m_minLogLevel;
            m_outputStreams.push_back(std::pair<std::ostream*, Severity>(stream, severity));
        }

        void Logger::flush() {
            while (!m_queue.IsEmpty()) {
                LogEntry entry = m_queue.Pop();
                std::stringstream sstream;

                //TODO: Write the log time in a more elegant manner.
                time_t timestamp = std::chrono::system_clock::to_time_t(entry.when);
                char* time_buffer = ctime(&timestamp);
                time_buffer[strlen(time_buffer) - 1] = '\0';
                sstream << time_buffer << "\t";

                switch (entry.severity) {
                    case Severity::DEBUG:
                        sstream << "DEBUG";
                        break;
                    case Severity::INFO:
                        sstream << "INFO ";
                        break;
                    case Severity::WARNING:
                        sstream << "WARN ";
                        break;
                    case Severity::ERROR:
                        sstream << "ERROR";
                        break;
                    case Severity::FATAL:
                        sstream << "FATAL";
                        break;
                    default:
                        break;
                }

                sstream << " [" << entry.channel << "]\t" << entry.message;

                const std::string& result_log = sstream.str();
                for (std::vector<std::pair<std::ostream*, Severity>>::const_iterator it = m_outputStreams.begin(); it != m_outputStreams.end(); ++it) {

                    if (entry.severity < it->second) {
                        // message severity isn't high enough for this logger.
                        continue;
                    }

                    std::ostream& current_stream = *(it->first);

                    // catch an exception in case we get a broken pipe or something...
                    try {
                        current_stream << result_log << std::endl;
                    } catch (...) {
                        // do nothing.
                    }
                }
            }
        }

        void Logger::worker(Logger *me) {
            std::unique_lock<std::mutex> lk (me->m_runningMutex);

            while (true) {
                me->flush();

                if (std::cv_status::no_timeout == me->m_runningCondition.wait_for(lk, std::chrono::seconds(5))) {
                    break;
                }
            }
        }
    }
}
