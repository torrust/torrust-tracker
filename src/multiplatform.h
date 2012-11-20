
#include <stdint.h>

#ifdef WIN32
#include <winsock2.h>
#include <windows.h>
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
typedef struct hostent HOSTENT;
typedef struct sockaddr SOCKADDR;
typedef struct sockaddr_in SOCKADDR_IN;
typedef struct in_addr IN_ADDR;
typedef struct hostent HOSTENT;
typedef void* LPVOID;
typedef void (LPTHREAD_START_ROUTINE)(LPVOID);
typedef pthread_t HANDLE;
//#define IPPROTO_UDP 0 // no protocol set.. SOCK_DGRAM is enough.

#define min(a,b) (a > b ? b : a)

#endif

