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
/*
 * NOTE: keep this header after standard C/C++ headers
 */

#include <stdint.h>

#if defined (_WIN32) && !defined (WIN32)
#define WIN32
#elif defined (__APPLE__) || defined (__CYGWIN__)
#define linux
#endif

#define VERSION "1.0.2-dev"

#ifdef WIN32
#include <winsock2.h>
#include <WS2tcpip.h>
#include <windows.h>
#elif defined (linux)
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <netinet/ip.h>
#include <netinet/in.h>
#include <unistd.h>
#include <netdb.h>
#include <pthread.h>
#include <fcntl.h>
#include <arpa/inet.h>

#define SOCKET int
#define INVALID_SOCKET 0
#define SOCKET_ERROR -1
#define DWORD uint64_t
#define closesocket(s) close(s)
typedef struct hostent HOSTENT;
typedef struct sockaddr SOCKADDR;
typedef struct sockaddr_in SOCKADDR_IN;
typedef struct in_addr IN_ADDR;
typedef void* LPVOID;
typedef void (LPTHREAD_START_ROUTINE)(LPVOID);
typedef pthread_t HANDLE;

#endif

#ifdef WIN32
#define PLATFORM "Windows"
#elif defined  (__APPLE__)
#define PLATFORM "Apple"
#elif defined (__CYGWIN__)
#define PLATFORM "Cygwin"
#else
#define PLATFORM "Linux"
#endif
