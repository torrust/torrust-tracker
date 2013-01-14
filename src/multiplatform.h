/*
 *	Copyright © 2012 Naim A.
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

#include <stdint.h>

#ifdef WIN32
#include <winsock2.h>
#include <windows.h>
#define VERSION "1.0.0 (Windows)"
#elif defined (linux)
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/ip.h>
#include <netinet/in.h>
#include <unistd.h>
#include <netdb.h>
#include <pthread.h>

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

#define min(a,b) (a > b ? b : a)
#define VERSION "1.0.0-alpha (Linux)"
#endif

