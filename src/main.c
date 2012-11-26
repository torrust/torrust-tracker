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

int main(int argc, char *argv[])
{
	printf("UDP BitTorrentTracker %s\t\tCopyright: (C) 2012 Naim Abda.\n\n", VERSION);

#ifdef linux
	if (argc > 1)
	{
		if (strcmp(argv[1], "d") == 0)
		{
			pid_t pid;
			pid = fork ();

			if (pid < 0)
			{
				printf ("Failed to start daemon.\n");
				exit (EXIT_FAILURE);
			}
			if (pid > 0)
			{
				printf("Daemon Started; pid=%d.\n", pid);
				fclose (stdin);
				fclose (stdout);
				fclose (stderr);
				exit (EXIT_SUCCESS);
			}
		}
	}
#endif

#ifdef WIN32
	WSADATA wsadata;
	WSAStartup(MAKEWORD(2, 2), &wsadata);
#endif

	udpServerInstance usi;
	UDPTracker_init(&usi, 6969, 5);

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
