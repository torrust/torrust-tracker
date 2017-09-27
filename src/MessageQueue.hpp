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

#include <queue>
#include <mutex>

namespace UDPT
{
    namespace Utils {

        template<class T>
        class MessageQueue {
        public:
            MessageQueue() {}

            virtual ~MessageQueue() {}

            bool IsEmpty() const {
                return m_queue.empty();
            }

            T Pop() {
                m_queueMutex.lock();
                T val = m_queue.front();
                m_queue.pop();
                m_queueMutex.unlock();

                return val;
            }

            void Push(T obj) {
                m_queueMutex.lock();
                m_queue.push(obj);
                m_queueMutex.unlock();
            }

            size_t Count() const {
                return m_queue.size();
            }

        private:
            std::queue<T> m_queue;
            std::mutex m_queueMutex;
        };

    }
}
