#
#	Copyright © 2012 Naim A.
#
#	This file is part of UDPT.
#
#		UDPT is free software: you can redistribute it and/or modify
#		it under the terms of the GNU General Public License as published by
#		the Free Software Foundation, either version 3 of the License, or
#		(at your option) any later version.
#
#		UDPT is distributed in the hope that it will be useful,
#		but WITHOUT ANY WARRANTY; without even the implied warranty of
#		MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#		GNU General Public License for more details.
#
#		You should have received a copy of the GNU General Public License
#		along with UDPT.  If not, see <http://www.gnu.org/licenses/>.
#

win32: main.o tools.o udpTracker.o driver_sqlite.o
	gcc -static -O3 -o udpt.exe main.o tools.o udpTracker.o driver_sqlite.o -lsqlite3 -lws2_32

linux: main.o tools.o udpTracker.o driver_sqlite.o
	gcc -static -O3 -o udpt main.o tools.o udpTracker.o driver_sqlite.o -lsqlite3 -lpthreads

main.o:
	gcc -c -O3 -o main.o src/main.c
	
tools.o:
	gcc -c -O3 -o tools.o src/tools.c
	
udpTracker.o:
	gcc -c -O3 -o udpTracker.o src/udpTracker.c
	
driver_sqlite.o:
	gcc -O3 -c -o driver_sqlite.o src/db/driver_sqlite.c
	
.PHONY: clean
clean:
	rm -f udpt.exe main.o tools.o udpTracker.o driver_sqlite.o udpt