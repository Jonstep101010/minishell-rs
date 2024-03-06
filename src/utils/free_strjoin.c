#include "libft.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "utils.h"

#include <stdbool.h>

size_t	get_len(const int count, va_list ap);
void	free_n_null_internal(const int n, va_list copy);

// len is amount
static char	*join_strings_internal(const int count, va_list ap)
{
	char	*ret;
	char	*input;
	size_t	i;
	size_t	offset;
	va_list	copy;

	offset = 0;
	va_copy(copy, ap);
	ret = ft_calloc(get_len(count, ap), sizeof(char));
	if (!ret)
		return (NULL);
	i = count;
	while (i--)
	{
		input = va_arg(ap, char *);
		if (!input)
		{
			free(ret);
			va_end(ap);
			return (NULL);
		}
		ft_strlcat(ret + offset, input, get_len(0, copy) - offset);
		offset += ft_strlen(input);
	}
	return (ret);
}

char	*str_join(const int count, ...)
{
	va_list	args;
	char	*ret;

	if (count <= 0)
		return (NULL);
	va_start(args, count);
	ret = join_strings_internal(count, args);
	va_end(args);
	return (ret);
}

// provide a va_list of char **
static char	*join_strings_free_internal(const int count, va_list ap)
{
	char	*ret;
	void	**input;
	size_t	i;
	size_t	offset;
	va_list	copy;

	offset = 0;
	va_copy(copy, ap);
	ret = ft_calloc(get_len(count, ap), sizeof(char));
	if (!ret)
		return (NULL);
	i = count;
	while (i--)
	{
		input = va_arg(ap, void **);
		if (!input)
			return (va_end(ap), free(ret), NULL);
		if (*input)
		{
			ft_strlcat(ret + offset, *input, get_len(0, copy) - offset);
			offset += ft_strlen(*((char **)input));
		}
	}
	return (free_n_null_internal(count, copy), ret);
}

/**
 * @brief requires heap-allocation
 * @warning if called with incorrect arguments, caller must free the input
 * @param count number of strings to join
 * @param ... always call with address of strings
 * @return char* joined string
 */
char	*free_strjoin(const int count, ...)
{
	va_list	args;
	char	*ret;

	if (count <= 0)
		return (NULL);
	va_start(args, count);
	ret = join_strings_free_internal(count, args);
	va_end(args);
	return (ret);
}
