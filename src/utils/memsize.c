#include <stddef.h>

size_t	arr_len_size(void *arr, size_t size)
{
	size_t	len;
	char	*byte_ptr;

	byte_ptr = (char*)arr;
	if (!byte_ptr || !size)
		return (0);
	len = 0;
	while (byte_ptr && *(void**)(byte_ptr + len * size))
		len++;
	return (len);
}
