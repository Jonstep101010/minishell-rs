#include "libft.h"
#include "minishell.h"
#include <stdbool.h>
#include <stddef.h>

static int	echo_opt(char **string)
{

}

static int	echo_default(char **string, size_t writelen, size_t n_args)
{
		while (string + writelen && *(string + writelen) && writelen <= n_args)
	{
		ft_printf(*(string + writelen));
		if (writelen < n_args - 1)
			write(1, " ", 1);
		writelen++;
	}
}

int	echo(char **string)
{
	size_t	n_args;
	size_t	writelen;
	int		size;
	bool	opt;

	if (!string || !*string)
		return (write(1, "\n", 1));
	n_args = arr_len(string);
	opt = false;
	writelen = 0;
	size = 0;
	if (ft_strncmp("-n", *string, 2) == 0)
	{
		opt = true;
		writelen = 1;
	}
	while (string + writelen && *(string + writelen) && writelen <= n_args)
	{
		size += ft_printf(*(string + writelen));
		if (writelen < n_args - 1)
			size += write(1, " ", 1);
		writelen++;
	}
	if (!opt)
		size += write(1, "\n", 1);
	return (size);
}


