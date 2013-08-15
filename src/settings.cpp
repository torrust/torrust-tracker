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
#include <exception>
#include <cstdlib>
#include <iostream>
#include <fstream>
#include "tools.h"

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
			if (cil == 0 && (c == ';' || c == '#'))
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

		if (classN == "" || name == "" || value == "")
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

	string Settings::SettingClass::get (const string& name)
	{
		if (this->entries.find(name) == this->entries.end())
			return "";
		return this->entries[name];
	}

	inline static int _isTrue (string str)
	{
		int i,		// loop index
			len;	// string's length

		if (str == "")
			return -1;
		len = str.length();
		for (i = 0;i < len;i++)
		{
			if (str[i] >= 'A' && str[i] <= 'Z')
			{
				str[i] = (str[i] - 'A' + 'a');
			}
		}
		if (str.compare ("yes") == 0)
			return 1;
		if (str.compare ("no") == 0)
			return 0;
		if (str.compare("true") == 0)
			return 1;
		if (str.compare ("false") == 0)
			return 0;
		if (str.compare("1") == 0)
			return 1;
		if (str.compare ("0") == 0)
			return 0;
		return -1;
	}

	bool Settings::SettingClass::getBool(const string& name)
	{
		string v = this->get(name);
		int r = _isTrue(v);
		if (r == 0 || r == 1)
			return (bool)r;
		throw SettingsException("Invalid boolean value.");
	}

	bool Settings::SettingClass::getBool (const string& key, bool defaultValue)
	{
		try {
			return this->getBool(key);
		} catch (SettingsException &e)
		{ }

		return defaultValue;
	}

	void Settings::SettingClass::getIPs (const string& key, list<SOCKADDR_IN> &ip)
	{
		string v = this->get(key) + " ";	// add padding for last entry.
		// expect a.b.c.d[:port], IPv4 only supported with BEP-15.

		string::size_type s, e;
		s = e = 0;
		char c;
		for (string::size_type i = 0;i < v.length();i++)
		{
			c = v[i];
			if (isspace(c) != 0 || c == ';' || c == ',')
			{
				if (s == e)
					s = e = i;
				else
				{
					string addr = v.substr(s, (e - s) + 1);
					SOCKADDR_IN saddr;
					saddr.sin_family = AF_INET;
					saddr.sin_addr.s_addr = 0L;
					saddr.sin_port = (6969);

					{
						uint8_t b;	// temporary container for IP byte
						uint16_t port;
						uint32_t ip;
						unsigned i,		// loop index
							stage;	// 0,1,2,3=IP[a.b.c.d], 4=port

						ip = 0;
						b = 0;
						stage = 0;
						for (i = 0;i < addr.length();i++)
						{
							if (addr[i] >= '0' && addr[i] <= '9')
							{
								if (stage <= 3)
								{
									b *= 10;
									b += (addr[i] - '0');
								}
								else if (stage == 4)
								{
									port *= 10;
									port += (addr[i] - '0');
								}
							}
							else if (addr[i] == '.' && stage < 3)
							{
								stage ++;
								ip *= 256;
								ip += b;
								b = 0;
							}
							else if (addr[i] == ':')
							{
								stage++;
								port = 0;

								ip *= 256;
								ip += b;
							}
						}

						if (stage == 3) // port not provided.
						{
							port = 6969;
							// add last byte.
							ip *= 256;
							ip += b;
						}
						saddr.sin_addr.s_addr = m_hton32(ip);
						saddr.sin_port = m_hton16(port);
					}

					ip.push_back(saddr);

					s = e = i + 1;
				}
			}
			else
			{
				e = i;
			}
		}
	}

	int Settings::SettingClass::getInt (const string& key, int def)
	{
		string v = this->get (key);
		if (v.length() == 0)
			return def;
		return std::atoi(v.c_str());
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
