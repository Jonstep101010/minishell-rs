#include "libft.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void	free_n(int n, ...)
{
	va_list	args;
	int		i;

	va_start(args, n);
	i = 0;
	while (i < n)
	{
		free(va_arg(args, void *));
		i++;
	}
	va_end(args);
}

/**
 * @brief requires heap-allocation
 *
 * @param count number of strings to join
 * @param ... strings to join
 * @return char* joined string
 */
char	*free_strjoin(int count, ...)
{
	va_list	args;
	char	*ret;
	char	*tmp;
	char	*arg;
	int		i;

	va_start(args, count);
	ret = va_arg(args, char *);
	i = 1;
	while (i < count)
	{
		arg = va_arg(args, char *);
		if (!ret || !arg)
		{
			free_n(2, ret, arg);
			while (++i < count)
				free(va_arg(args, char *));
			return (va_end(args), NULL);
		}
		tmp = ft_strjoin(ret, arg);
		free_n(2, ret, arg);
		ret = tmp;
		i++;
	}
	return (va_end(args), ret);
}
