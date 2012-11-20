/*
 ============================================================================
 Name        : udpBitTorrentTracker.c
 Author      : 
 Version     :
 Copyright   : 
 Description : Hello World in C, Ansi-style
 ============================================================================
 */

#include <stdio.h>
#include <stdlib.h>

//#include <winsock2.h>
//#include <windows.h>
#include "multiplatform.h"

#include "udpTracker.h"
#include "tools.h"
#include <math.h>
#include <time.h>

int main(void)
{

	printf("UDP BitTorrentTracker\t\tCopyright: (C) 2012 Naim Abda.\n\n");

#ifdef WIN32
	WSADATA wsadata;
	WSAStartup(MAKEWORD(2, 2), &wsadata);
#endif

	udpServerInstance usi;
	UDPTracker_init(&usi, 6969, 1);

	if (UDPTracker_start(&usi) != 0)
	{
		printf("Error While trying to start server.");
		return 1;
	}

//	system("pause");
	printf("Press Any key to exit...\n");
	int i;
	for (i = 0;i < usi.thread_count;i++)
		pthread_join (usi.threads[i], NULL);
	printf("\n");

//	UDPTracker_destroy(&usi);

#ifdef WIN32
	WSACleanup();
#endif

	return 0;
}
