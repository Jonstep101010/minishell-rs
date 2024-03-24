#include "arr_utils.h"
#include "libft.h"
#include "tokens.h"

static int	is_n_arg(char const *arg)
{
	if (*arg == '-')
	{
		arg++;
		while (*arg == 'n')
		{
			arg++;
			if (*arg == '\0')
				return (1);
		}
	}
	return (0);
}

// 0 is case of -n flag running and a string with -nsomething -> "\n"
// 1 is case of normal -n flag -> no newline
// 2 is case of no -n flag -> "\n"
static void	echo_default(const char *const *cmd_args)
{
	int		i;
	int		flag;
	int		n_pos;

	i = 0;
	flag = 0;
	while (cmd_args[i] && is_n_arg(cmd_args[i]) == 1)
		i++;
	if (i == 0)
		flag = 2;
	if (i > 0)
		flag = 1;
	n_pos = i;
	while (cmd_args[i])
	{
		if (flag == 1 && ft_strncmp(cmd_args[i], "-n", 2) == 0
			&& cmd_args[i][2] != '\0' && i != n_pos + 1)
			flag = 0;
		printf("%s", cmd_args[i]);
		if (cmd_args[i + 1])
			printf(" ");
		i++;
	}
	if (flag != 1)
		printf("\n");
}

int	echo(t_shell *nullable, t_token *token)
{
	const char	**args = (const char **)get_cmd_arr_token(token);

	(void)nullable;
	if (*(args + 1))
		echo_default(&args[1]);
	else
		write(1, "\n", 1);
	arr_free((char **)args);
	return (0);
}
