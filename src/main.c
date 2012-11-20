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

#include "multiplatform.h"

#include "udpTracker.h"
#include "tools.h"
#include <math.h>
#include <time.h>

int main(void)
{
	printf("UDP BitTorrentTracker %s\t\tCopyright: (C) 2012 Naim Abda.\n\n", VERSION);

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

	printf("Press Any key to exit.\n");

	getchar ();

	printf("\nGoodbye.\n");
	UDPTracker_destroy(&usi);

#ifdef WIN32
	WSACleanup();
#endif

	return 0;
}
