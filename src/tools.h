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

#ifndef TOOLS_H_
#define TOOLS_H_

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Swaps Bytes:
 * example (htons):
 * 	short a = 1234;
 * 	short b;
 * 	m_byteswap (&b, &a, sizeof(a));
 */
void m_byteswap (void *dest, void *src, int sz);

uint16_t m_hton16(uint16_t n);

uint32_t m_hton32 (uint32_t n);

uint64_t m_hton64 (uint64_t n);

void to_hex_str (const uint8_t *hash, char *data);

#ifdef __cplusplus
}
#endif

#endif /* TOOLS_H_ */
