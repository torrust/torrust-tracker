#pragma once


namespace UDPT
{
	class UDPTException
	{
	public:
		UDPTException(const char* errorMsg = "", int errorCode = 0) : m_error(errorMsg), m_errorCode(errorCode)
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
}
