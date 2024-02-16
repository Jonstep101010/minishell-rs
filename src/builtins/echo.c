#include "libft.h"
#include "utils.h"

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
	size_t	len;

	n_args = arr_len((const char **)cmd_arr);
	if (writelen == 1 && (cmd_arr + writelen))
	{
		while (cmd_arr + writelen && is_n_arg(*(cmd_arr + writelen)))
			writelen++;
	}
	while (cmd_arr + writelen && *(cmd_arr + writelen) && writelen <= n_args)
	{
		len = 0;
		while (cmd_arr[writelen][len] != '\0')
			len++;
		write(1, cmd_arr[writelen], len);
		if (writelen < n_args - 1)
			write(1, " ", 1);
		writelen++;
	}
	return (0);
}

int	echo(const char *cmd, const char **args, const char **envp)
{
	(void)cmd;
	(void)envp;
	if (!args || !*args)
		return (write(1, "\n", 1), 0);
	if (occurs(*(args + 1), "-n")
		&& ft_strncmp(*(args + 1), "-n", 2) == 0)
			return (echo_default(args + 1, 1), 0);
	echo_default(&args[1], 0);
	write(1, "\n", 1);
	return (0);
}


