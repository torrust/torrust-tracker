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

#pragma once

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

/**
 * Initializes the settings type.
 * @param s Pointer to settings to initialize.
 * @param filename the settings filename.
 */
void settings_init (Settings *s, const char *filename);

/**
 * Loads settings from file
 * @param s pointer to settings type
 * @return 0 on success, otherwise non-zero.
 */
int settings_load (Settings *s);

/**
 * Saves settings to file.
 * @param s Pointer to settings.
 * @return 0 on success; otherwise non-zero.
 */
int settings_save (Settings *s);

/**
 * Destroys the settings "object"
 * @param s Pointer to settings.
 */
void settings_destroy (Settings *s);

/**
 * Gets the requested SettingClass.
 * @param s Settings Object.
 * @param classname The name of the class to find (case sensitive).
 * @return a pointer to the found class, or NULL if not found.
 */
SettingClass* settings_get_class (Settings *s, const char *classname);

char* settingclass_get (SettingClass *s, const char *name);

int settingclass_set (SettingClass *s, const char *name, const char *value);

/**
 * Gets a setting from a Settings type.
 * @param s Pointer to a setting type.
 * @param class The class of the requested setting.
 * @param name The name of the requested setting.
 * @return The value for the requested setting, NULL if not available.
 */
char* settings_get (Settings *s, const char *classn, const char *name);

/**
 * Sets a setting in a settings type.
 * @param s Pointer to settings type.
 * @param class The class of the setting.
 * @param name The name of the setting.
 * @param value The value to set for the setting.
 * @return 0 on success, otherwise non-zero.
 */
int settings_set (Settings *s, const char *classn, const char *name, const char *value);
