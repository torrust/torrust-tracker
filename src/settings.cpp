/*
 *
 *	Copyright © 2012,2013 Naim A.
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

#include "settings.hpp"
#include <string.h> // still primitive - need for strlen()
#include <ctype.h> // need for isspace()

#include <iostream>
#include <fstream>

using namespace std;

namespace UDPT
{
	Settings::SettingClass* Settings::getClass(const string classname)
	{
		if (classname == "")
			return NULL;

		map<string, SettingClass*>::iterator it;
		it = this->classes.find(classname);

		if (it == this->classes.end())
			return NULL;
		else
			return it->second;

		return NULL;
	}

	Settings::Settings (const string filename)
	{
		this->filename = filename;
		this->classes.clear();
	}

static
void _settings_clean_string (char **str)
{
	int len,
		i,
		offset;

	len = strlen(*str);

	//strip leading whitespaces.
	offset = 0;
	for (i = 0;i < len;i++)
	{
		if (isspace(*str[i]) == 0)
			break;
		offset++;
	}

	(*str) += offset;
	len -= offset;

	for (i = len - 1;i >= 0;i--)
	{
		if (isspace( (*str)[i] ) != 0)
		{
			(*str)[i] = '\0';
		}
		else
			break;
	}
}

	void Settings::parseSettings (char *data, int len)
	{
		char *className, *key, *value;
		int i,
			cil; // cil = Chars in line.
		char c;

		className = key = value = NULL;
		cil = 0;

		for (i = 0;i < len;i++)
		{
			c = data[i];
			if (c == '\n')
			{
				cil = 0;
				continue;
			}
			if (cil == 0 && c == ';')
			{
				while (i < len)
				{
					if (data[i] == '\n')
						break;
					i++;
				}
				continue;
			}
			if (isspace(c) != 0 && cil == 0)
			{
				continue;
			}
			if (cil == 0 && c == '[')
			{
				className = (char*)(i + data + 1);
				while (i < len)
				{
					if (data[i] != ']')
					{
						i++;
						continue;
					}
					data[i] = '\0';
					break;
				}
				continue;
			}

			if (isgraph(c) != 0 && cil == 0) // must be a key.
			{
				key = (char*)(i + data);
				while (i < len)
				{
					if (data[i] == '\n')
					{
						key = NULL;
						break;
					}
					if (data[i] == '=')
					{
						data[i] = '\0';
						value = (char*)(data + i + 1);
						while (i < len)
						{
							if (data[i] == '\n')
							{
								data[i] = '\0';

								_settings_clean_string(&key);
								_settings_clean_string(&value);

	//							printf("KEY: '%s'\tVALUE: '%s'\n", key, value);

								// add to settings...
								this->set (className, key, value);

								cil = 0;
								break;
							}
							i++;
						}
						break;
					}
					i++;
				}
				continue;
			}

			if (isgraph(c) != 0)
			{
				cil++;
			}
		}
	}

	bool Settings::load()
	{
		int len;
		char *buffer;

		fstream cfg;
		cfg.open(this->filename.c_str(), ios::in | ios::binary);

		if (!cfg.is_open())
			return false;

		cfg.seekg(0, ios::end);
		len = cfg.tellg();
		cfg.seekg(0, ios::beg);

		buffer = new char [len];
		cfg.read(buffer, len);
		cfg.close();

		this->parseSettings(buffer, len);

		delete[] buffer;

		return true;
	}

	bool Settings::save ()
	{
		SettingClass *sclass;

		fstream cfg (this->filename.c_str(), ios::binary | ios::out);
		if (!cfg.is_open())
			return false;

		cfg << "; udpt Settings File - Created Automatically.\n";

		map<string, SettingClass*>::iterator it;
		for (it = this->classes.begin();it != this->classes.end();it++)
		{
			sclass = it->second;
			cfg << "[" << it->first.c_str() << "]\n";

			map<string, string>::iterator rec;
			for (rec = sclass->entries.begin();rec != sclass->entries.end();rec++)
			{
				cfg << rec->first.c_str() << "=" << rec->second.c_str() << "\n";
			}

			cfg << "\n";
		}
		cfg.close();

		return 0;
	}

	Settings::~Settings()
	{
		map<string, SettingClass*>::iterator it;
		for (it = this->classes.begin();it != this->classes.end();it++)
		{
			SettingClass *sc = it->second;
			delete sc;
		}
		this->classes.clear();
	}

	string Settings::get (const string classN, const string name)
	{
		SettingClass *c;

		c = this->getClass(classN);
		if (c == NULL)
			return "";
		return c->get(name);
	}

	bool Settings::set (const string classN, const string name, const string value)
	{
		SettingClass *c;

		if (classN == "" || name == "")
			return false;

		c = this->getClass (classN);

		if (c == NULL)
		{
			c = new SettingClass(classN);
			this->classes.insert(pair<string, SettingClass*>(classN, c));
		}

		return c->set (name, value);
	}

	Settings::SettingClass::SettingClass(const string cn)
	{
		this->className = cn;
	}

	string Settings::SettingClass::get (const string name)
	{
		if (this->entries.find(name) == this->entries.end())
			return "";
		return this->entries[name];
	}

	map<string, string>* Settings::SettingClass::getMap()
	{
		return &this->entries;
	}

	bool Settings::SettingClass::set (const string name, const string value)
	{
		pair<map<string, string>::iterator, bool> r;
		r = this->entries.insert(pair<string, string>(name, value));
		if (!r.second)
		{
			r.first->second = value;
		}

		return true;
	}
};
