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

#include <stdio.h>
#include <stdlib.h>

#include "multiplatform.h"
#include "udpTracker.h"
#include "tools.h"
#include <math.h>
#include <time.h>
#include <string.h>

static void _print_usage ()
{
	printf ("Usage: udpt <udp-port>\n"
			"\tDefault port is 6969.\n");
}

int main(int argc, char *argv[])
{
	printf("UDP Tracker (UDPT) %s\nCopyright: (C) 2012 Naim Abda <naim94a@gmail.com>\n\n", VERSION);

#ifdef WIN32
	WSADATA wsadata;
	WSAStartup(MAKEWORD(2, 2), &wsadata);
#endif

	uint16_t port = 6969;
	if (argc <= 1)
	{
		_print_usage ();
	}
	else if (argc == 2)
	{
		port = atoi(argv[1]);
		printf("selected port=%u\n", port);
	}

	udpServerInstance usi;
	UDPTracker_init(&usi, port, 5);

	int r = UDPTracker_start(&usi);
	if (r != 0)
	{
		printf("Error While trying to start server.\n");
		switch (r)
		{
		case 1:
			printf("Failed to create socket.\n");
			break;
		case 2:
			printf("Failed to bind socket.\n");
			break;
		default:
			printf ("Unknown Error\n");
			break;
		}
		goto cleanup;
	}

	printf("Press Any key to exit.\n");

	getchar ();

cleanup:
	printf("\nGoodbye.\n");

	UDPTracker_destroy(&usi);

#ifdef WIN32
	WSACleanup();
#endif

	return 0;
}
