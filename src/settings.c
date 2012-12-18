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

#include "settings.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

SettingClass* settings_get_class (Settings *s, char *classname)
{
	if (s == NULL || classname == NULL)
		return NULL;
	int i;
	for (i = 0;i < s->class_count;i++)
	{
		if (strcmp(classname, s->classes[i].classname) == 0)
		{
			return &s->classes[i];
		}
	}

	return NULL;
}

void settings_init (Settings *s, char *filename)
{
	s->buffer = NULL;
	s->filename = filename;
	s->classes = NULL;
	s->class_count = s->class_size = 0;
}

static
void _settings_clean_string (char **str)
{
	int len = strlen(*str);

	//strip leading whitespaces.
	int i, offset = 0;
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

static
void _settings_parser (Settings *s, char *data, int len)
{
	char *className = NULL;
	char *key = NULL;
	char *value = NULL;
	int i, cil = 0; // cil = Chars in line.
	char c;
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
							settings_set(s, className, key, value);

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

int settings_load (Settings *s)
{
	if (s->buffer != NULL)
	{
		free (s->buffer);
		s->buffer = NULL;
	}

	// ini file format.
	FILE *f = fopen(s->filename, "rb");
	if (f == NULL)
		return 1;
	fseek (f, 0, SEEK_END);
	int len = ftell(f);
	fseek(f, 0, SEEK_SET);

	s->buffer = malloc (len);
	char *buffer = s->buffer;

	char tmp [512];
	int r = 0, offset = 0;
	while (!feof(f) && !ferror(f))
	{
		r = fread (tmp, 1, 512, f);
		int i;
		for (i = 0;i < r;i++)
		{
			buffer[offset + i] = tmp[i];
		}
		offset += r;
	}

	fclose (f);
//	printf("File loaded into buffer. size=%d\n", len);
	_settings_parser (s, buffer, len);

	return 0;
}

int settings_save (Settings *s)
{
	char buffer [2048];

	FILE *f = fopen(s->filename, "wb");
	fprintf(f, "; udpt Settings File - Created Automatically.\n");
	setbuf(f, buffer);

	int c, e;
	SettingClass *class;
	for (c = 0;c < s->class_count;c++)
	{
		class = &s->classes[c];
		fprintf(f, "[%s]\n", class->classname);

		for (e = 0;e < class->entry_count;e++)
		{
			fprintf(f, "%s=%s\n", class->entries[e].key, class->entries[e].values);
		}

		fprintf(f, "\n");
	}

	fclose (f);

	return 0;
}

void settings_destroy (Settings *s)
{
	if (s->classes != NULL)
	{

		int i;
		for (i = 0;i < s->class_count;i++)
		{
			if (s->classes[i].entries != NULL)
				free (s->classes[i].entries);
		}

		free (s->classes);
	}
	if (s->buffer != NULL)
	{
		free (s->buffer);
		s->buffer = NULL;
	}
}

char* settings_get (Settings *s, char *class, char *name)
{
	if (s == NULL || class == NULL || name == NULL)
		return NULL;

	SettingClass *c = settings_get_class (s, class);
	return settingclass_get (c, name);
}

int settings_set (Settings *s, char *class, char *name, char *value)
{
	if (s == NULL || class == NULL || name == NULL)
		return 1;

	SettingClass *c = settings_get_class (s, class);

	if (c == NULL)
	{
		if (s->class_count + 1 >= s->class_size)
		{
			int ns = s->class_size + 1;
			SettingClass *sc = realloc (s->classes, sizeof(SettingClass) * ns);
			if (sc == NULL)
				return 1;
			s->classes = sc;
			s->class_size = ns;
		}

		c = &s->classes[s->class_count];
		s->class_count++;

		c->classname = class;
		c->entries = NULL;
		c->entry_size = c->entry_count = 0;

	}

	return settingclass_set (c, name, value);
}

char* settingclass_get (SettingClass *c, char *name)
{
	if (c == NULL)
		return NULL;

	KeyValue *kv;
	int i;
	for (i = 0;i < c->entry_count;i++)
	{
		kv = &c->entries[i];
		if (strcmp(kv->key, name) == 0)
			return kv->values;
	}
	return NULL;
}

int settingclass_set (SettingClass *c, char *name, char *value)
{

	int i;
	for (i = 0;i < c->entry_count;i++)
	{
		if (strcmp(name, c->entries[i].key) == 0)
		{
			c->entries[i].values = value;
			return 0;
		}
	}

	if (c->entry_count + 1 >= c->entry_size)
	{
		int ns = c->entry_size + 5;
		KeyValue *n = realloc (c->entries, sizeof(KeyValue) * ns);
		if (n == NULL)
			return 1;
		c->entries = n;
		c->entry_size = ns;
	}

	int ni = c->entry_count;
	c->entry_count++;

	c->entries[ni].key = name;
	c->entries[ni].values = value;

	return 0;
}
