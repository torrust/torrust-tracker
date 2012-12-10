/*
 *
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

typedef struct {
	char *key;
	char *values;
} KeyValue;

typedef struct {
	char *classname;
	KeyValue *entries;
	uint32_t entry_count, entry_size;
} SettingClass;

typedef struct {
	char *filename;

	SettingClass *classes;
	uint32_t class_count, class_size;

	char *buffer;
} Settings;

void settings_init (Settings *, char *filename);

int settings_load (Settings *);

int settings_save (Settings *);

void settings_destroy (Settings *);

char* settings_get (Settings *, char *class, char *name);

int settings_set (Settings *, char *class, char *name, char *value);
