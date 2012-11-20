/*
 * tools.h
 *
 *  Created on: Nov 18, 2012
 *      Author: Naim
 */

#ifndef TOOLS_H_
#define TOOLS_H_

#include <stdint.h>

#define m_hton16(n) htons(n)

uint32_t m_hton32 (uint32_t n);

uint64_t m_hton64 (uint64_t n);

#endif /* TOOLS_H_ */
