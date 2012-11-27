/*
 * tools.h
 *
 *  Created on: Nov 18, 2012
 *      Author: Naim
 */

#ifndef TOOLS_H_
#define TOOLS_H_

#include <stdint.h>

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

#endif /* TOOLS_H_ */
