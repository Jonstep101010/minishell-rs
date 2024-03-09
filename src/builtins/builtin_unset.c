#include "environment.h"
#include "libft.h"
#include "tokens.h"
#include "utils.h"
#include "struct.h"

static int	unset_internal(const char *const *args, char **env)
{
	int	index;

	while (*args)
	{
		if (!check_valid_key(*args))
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

/**
 * @brief lookup varname if it is valid, remove it from envp
 * @param args to read
 * @param envp to modify
 * @return 1 on invalid varname, 0 on success
 */
int	unset(t_shell *shell, t_token *token)
{
	const char *const	*args = (const char *const *)token->command;

	if (!shell->env || !*(args + 1) || !*shell->env)
		return (0);
	return (unset_internal(args + 1, shell->env));
}
