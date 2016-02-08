#
#	Copyright © 2012,2013 Naim A.
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

objects = main.o udpTracker.o database.o driver_sqlite.o \
	tools.o httpserver.o webapp.o tracker.o
target = udpt
CXXFLAGS = -DBOOST_LOG_DYN_LINK

%.o: src/%.c
	$(CC) -c -o $@ $< $(CFLAGS)
%.o: src/%.cpp
	$(CXX) -g -std=gnu++11 -c -o $@ $< $(CXXFLAGS)
%.o: src/db/%.cpp
	$(CXX) -g -std=gnu++11 -c -o $@ $< $(CXXFLAGS)
%.o: src/http/%.cpp
	$(CXX) -g -std=gnu++11 -c -o $@ $< $(CXXFLAGS)
all: $(target)
	
$(target): $(objects)
	@echo Linking...
	$(CXX) -O3 -o $(target) $(objects) $(LDFLAGS) -lboost_program_options -lsqlite3 -lpthread -lboost_thread -lboost_system -lboost_log
	@echo Done.
clean:
	@echo Cleaning Up...
	$(RM) $(objects) $(target)
	@echo Done.

