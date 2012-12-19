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
#include "settings.h"

static void _print_usage ()
{
	printf ("Usage: udpt [<configuration file>]\n");
}

int main(int argc, char *argv[])
{
	printf("UDP Tracker (UDPT) %s\nCopyright: (C) 2012 Naim Abda <naim94a@gmail.com>\n", VERSION);
	printf("Build Date: %s\n\n", __DATE__);

#ifdef WIN32
	WSADATA wsadata;
	WSAStartup(MAKEWORD(2, 2), &wsadata);
#endif

	char *config_file = "udpt.conf";

	if (argc <= 1)
	{
		_print_usage ();
	}

	Settings settings;
	udpServerInstance usi;

	settings_init (&settings, config_file);
	if (settings_load (&settings) != 0)
	{
		const char strDATABASE[] = "database";
		const char strTRACKER[] = "tracker";
		// set default settings:

		settings_set (&settings, strDATABASE, "driver", "sqlite3");
		settings_set (&settings, strDATABASE, "file", "tracker.db");

		settings_set (&settings, strTRACKER, "port", "6969");
		settings_set (&settings, strTRACKER, "threads", "5");
		settings_set (&settings, strTRACKER, "allow_remotes", "yes");
		settings_set (&settings, strTRACKER, "allow_iana_ips", "yes");
		settings_set (&settings, strTRACKER, "announce_interval", "1800");
		settings_set (&settings, strTRACKER, "cleanup_interval", "120");

		settings_save (&settings);
		printf("Failed to read from '%s'. Using default settings.\n", config_file);
	}

	UDPTracker_init(&usi, &settings);

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
	settings_destroy (&settings);
	UDPTracker_destroy(&usi);

#ifdef WIN32
	WSACleanup();
#endif

	return 0;
}
