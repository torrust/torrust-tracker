#include "tools.h"

uint64_t m_hton64 (uint64_t n)
{
	uint64_t r = m_hton32 (n & 0xffffffff);
	r <<= 32;
	r |= m_hton32 (n & 0xffffffff00000000);
	return r;
}

uint32_t m_hton32 (uint32_t n)
{
	uint32_t r = m_hton16 (n & 0xffff);
	r <<= 16;
	r |= m_hton16 (n & 0xffff0000);
	return r;
}
