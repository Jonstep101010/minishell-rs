#include "libft.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "utils.h"

// do not pass in <= 0 for n
// function to free gets a va_list and a count, frees all non-null strings
void	free_n_null_internal(const int n, va_list copy)
{
	void	**input;
	int		i;

	if (n <= 0)
		return ;
	input = NULL;
	i = n;
	while (i > 0)
	{
		input = va_arg(copy, void **);
		if (input && *input)
		{
			free(*input);
			*input = NULL;
		}
		i--;
	}
	va_end(copy);
}

// always pass with address of pointers
void	free_n_null(int n, ...)
{
	va_list	args;

	va_start(args, n);
	free_n_null_internal(n, args);
}

void	free_n(int n, ...)
{
	va_list	args;
	void	*input;
	int		i;

	if (n <= 0)
		return ;
	va_start(args, n);
	i = n;
	while (i--)
	{
		input = va_arg(args, void *);
		if (input)
			free(input);
	}
	va_end(args);
}

size_t	get_len(const int count, va_list ap)
{
	size_t			i;
	static size_t	length;
	char			*input;
	va_list			ap_start;

	if (count <= 0)
		return (length);
	i = count;
	length = 0;
	va_copy(ap_start, ap);
	while (i--)
	{
		input = va_arg(ap_start, char *);
		if (!input)
			break ;
		length += ft_strlen(input);
	}
	length++;
	va_end(ap_start);
	return (length);
}

// second string needs to be heap allocated
char	*free_second_join(char const *s1, char *s2)
{
	char	*ret;

	if (!s1)
		return (free(s2), NULL);
	if (!s2)
		return (NULL);
	ret = ft_strjoin(s1, s2);
	free(s2);
	return (ret);
}
