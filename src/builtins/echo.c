#include "libft.h"
#include "builtins.h"
#include <stddef.h>

char	*occurs_exclusively(const char *expected, const char *actual);

static int	is_n_arg(const char *arg)
{
	if (occurs(arg, "-n")
		&& ft_strncmp(arg, "-n", 2) == 0)
			return (1);
	return (0);
}

static int	echo_default(const char **cmd_arr, size_t writelen)
{
	size_t	n_args;

	n_args = arr_len(cmd_arr);
	if (writelen == 1 && (cmd_arr + writelen))
	{
		while (cmd_arr + writelen && is_n_arg(*(cmd_arr + writelen)))
			writelen++;
	}
	while (cmd_arr + writelen && *(cmd_arr + writelen) && writelen <= n_args)
	{
		ft_printf(*(cmd_arr + writelen));
		if (writelen < n_args - 1)
			write(1, " ", 1);
		writelen++;
	}
	return (0);
}

int	echo(const char **cmd_arr)
{
	if (!cmd_arr || !*cmd_arr || !*(cmd_arr + 1))
		return (write(1, "\n", 1));
	if (occurs(*(cmd_arr + 1), "-n")
		&& ft_strncmp(*(cmd_arr + 1), "-n", 2) == 0)
			return (echo_default(cmd_arr + 1, 1));
	echo_default((cmd_arr + 1), 0);
	write(1, "\n", 1);
	return (0);
}


