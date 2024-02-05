#include "libft.h"
#include "utils.h"

static int	is_n_arg(const char *arg)
{
	if (occurs(arg, "-n")
		&& ft_strncmp(arg, "-n", 2) == 0)
			return (1);
	return (0);
}

static int	echo_default(char **cmd_arr, size_t writelen)
{
	size_t	n_args;

	n_args = arr_len((const char **)cmd_arr);
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

size_t	echo(const char *cmd, char **args, char **envp)
{
	(void)cmd;
	(void)envp;
	if (!args || !*args)
		return (write(1, "\n", 1));
	if (occurs(*args, "-n")
		&& ft_strncmp(*args, "-n", 2) == 0)
			return (echo_default(args, 1));
	echo_default(args, 0);
	write(1, "\n", 1);
	return (0);
}


