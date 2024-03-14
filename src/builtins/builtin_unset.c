#include "environment.h"
#include "tokens.h"
#include "utils.h"
#include "struct.h"
#include "libft.h"

static bool	check_illegal_char(const char *str)
{
	while (*str)
	{
		if (ft_strchr("?=;$.:><[]()/+-{}*#@!^", *str))
			return (true);
		str++;
	}
	return (false);
}

static int	unset_internal(const char *const *args, char **env)
{
	int	index;

	while (*args)
	{
		if (!check_valid_key(*args) || check_illegal_char(*args))
		{
			eprint("unset: %s: invalid option", *args);
			return (1);
		}
		index = get_index_env(env, *args);
		if (index >= 0 && env[index])
			rm_str_arr(env, env[index]);
		args++;
	}
	return (0);
}

int	builtin_unset(t_shell *shell, t_token *token)
{
	const char *const	*args = (const char *const *)token->command;

	if (!shell->env || !*(args + 1) || !*shell->env)
		return (0);
	return (unset_internal(args + 1, shell->env));
}
