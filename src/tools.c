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
