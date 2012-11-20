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
