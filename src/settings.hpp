/*
 *
 *	Copyright © 2012-2016 Naim A.
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
#include <map>
#include <string>
#include <list>
#include "multiplatform.h"
using namespace std;

namespace UDPT
{
	class Settings
	{
	public:
		class SettingsException : public std::exception
		{
		public:
			SettingsException (const char *str)
			{
				this->str = str;
			}

			const char * what ()
			{
				return str;
			}

		private:
			const char *str;
		};

		class SettingClass
		{
		public:
			SettingClass (const string className);
			bool set (const string key, const string value);
			string get (const string& key);
			bool getBool (const string& key);
			bool getBool (const string& key, bool defaultValue);
			int getInt (const string& key, int def = -1);
			map<string, string>* getMap ();
			void getIPs (const string& key, list<SOCKADDR_IN> &ip);
		private:
			friend class Settings;
			string className;
			map<string, string> entries;
		};

		/**
		 * Initializes the settings type.
		 * @param filename the settings filename.
		 */
		Settings (const string filename);

		/**
		 * Gets a setting from a Settings type.
		 * @param class The class of the requested setting.
		 * @param name The name of the requested setting.
		 * @return The value for the requested setting, NULL if not available.
		 */
		SettingClass* getClass (const string name);

		/**
		 * Loads settings from file
		 * @return true on success, otherwise false.
		 */
		bool load ();

		/**
		 * Saves settings to file.
		 * @return true on success; otherwise false.
		 */
		bool save ();

		/**
		 * Sets a setting in a settings type.
		 * @param className The class of the setting.
		 * @param key The name of the setting.
		 * @param value The value to set for the setting.
		 * @return true on success, otherwise false.
		 */
		bool set (const string className, const string key, const string value);

		/**
		 * Gets the requested SettingClass.
		 * @param classname The name of the class to find (case sensitive).
		 * @return a pointer to the found class, or NULL if not found.
		 */
		string get (const string className, const string key);

		/**
		 * Destroys the settings "object"
		 */
		virtual ~Settings ();
	private:
		string filename;
		map<string, SettingClass*> classes;

		void parseSettings (char *data, int len);
	};
};
