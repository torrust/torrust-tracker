#pragma once

#include "multiplatform.h"

namespace UDPT
{
    class UDPTException
    {
    public:
        UDPTException(const char* errorMsg, int errorCode = 0) : m_error(errorMsg), m_errorCode(errorCode)
        {

        }

        UDPTException(int errorCode = 0) : m_errorCode(errorCode), m_error("")
        {
        }

        virtual const char* what() const
        {
            return m_error;
        }

        virtual int getErrorCode() const
        {
            return m_errorCode;
        }

        virtual ~UDPTException()
        {

        }

    protected:
        const char* m_error;
        const int m_errorCode;
    };

    class OSError : public UDPTException
    {
    public:
        OSError(int errorCode
#ifdef WIN32 
            = ::GetLastError()
#endif
            ) : UDPTException(errorCode)
        {
        }

        virtual ~OSError() {}

        const char* what() const
        {
            if (m_errorMessage.length() > 0)
            {
                return m_errorMessage.c_str();
            }

#ifdef WIN32 
            char *buffer = nullptr;
            DWORD msgLen = ::FormatMessageA(FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER, 0, m_errorCode, MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US), reinterpret_cast<LPSTR>(&buffer), 1, NULL);
            std::shared_ptr<void> formatStr = std::shared_ptr<void>(
                buffer,
                ::LocalFree);
            m_errorMessage = std::string(reinterpret_cast<char*>(formatStr.get()));

            return m_errorMessage.c_str();
#else 
            return "OSError";
#endif
        }
    private:
        // allow to generate a message only when needed.
        mutable std::string m_errorMessage;
    };
}
