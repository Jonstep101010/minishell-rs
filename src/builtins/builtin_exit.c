#include "struct.h"
#include "arr_utils.h"
#include <stdlib.h>
#include "utils.h"
#include "libft.h"

static bool		check_sign(const char *exit_code)
{
	if (*exit_code == '-' || *exit_code == '+')
	{
		if (*(exit_code + 1) == 0)
			return (false);
	}
	return (true);
}

static bool		check_exit_code(t_token *code_nullable)
{
	char	*exit_code;
	int		i;

	i = -1;
	if (code_nullable->command[1] && code_nullable->command[2])
		return (eprint("exit: too many arguments"), false);
	exit_code = code_nullable->command[1];
	if (ft_strlen(exit_code) == 1 && *exit_code == '0')
		return (true);
	while (exit_code[++i])
	{
		if (!ft_isdigit(exit_code[i]) && i != 0)
		{
			eprint("exit: numeric argument required");
			return (false);
		}
	}
	if (!check_sign(exit_code))
	{
		eprint("exit: numeric argument required");
		return (false);
	}
	return (true);
}

int		builtin_exit(t_shell *shell, t_token *code_nullable)
{
	uint8_t		exit_code;

	exit_code = shell->exit_status;
	if (code_nullable && code_nullable->command[1])
	{
		if (!check_exit_code(code_nullable))
			return (1);
		exit_code = ft_atol(code_nullable->command[1]);
	}
	eprint_single("exit\n", exit_code);
	if (shell->env)
		arr_free(shell->env);
	destroy_all_tokens(shell);
	free(shell);
	exit(exit_code);
}
