/*
 *	Copyright © 2012-2017 Naim A.
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

#include "tools.h"
#include "multiplatform.h"

void m_byteswap (void *dest, void *src, int sz)
{
    int i;
    for (i = 0;i < sz;i++)
    {
        ((char*)dest)[i] = ((char*)src)[(sz - 1) - i];
    }
}

uint16_t m_hton16(uint16_t n)
{
    uint16_t r;
    m_byteswap (&r, &n, 2);
    return r;
}

uint64_t m_hton64 (uint64_t n)
{
    uint64_t r;
    m_byteswap (&r, &n, 8);
    return r;
}

uint32_t m_hton32 (uint32_t n)
{
    uint64_t r;
    m_byteswap (&r, &n, 4);
    return r;
}


static const char hexadecimal[] = "0123456789abcdef";

void to_hex_str (const uint8_t *hash, char *data)
{
    int i;
    for (i = 0;i < 20;i++)
    {
        data[i * 2] = hexadecimal[hash[i] / 16];
        data[i * 2 + 1] = hexadecimal[hash[i] % 16];
    }
    data[40] = '\0';
}
