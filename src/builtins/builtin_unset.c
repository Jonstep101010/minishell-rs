#include "environment.h"
#include "libft.h"
#include "tokens.h"
#include "utils.h"
#include "struct.h"

static int	unset_internal(const char **args, char **envp)
{
	int	index;

	while (*args)
	{
		if (!check_valid_key(*args))
		{
			eprint("%s: %s: %s", "unset", *args, "invalid option");
			return (1);
		}
		index = find_key_env(envp, *args, ft_strlen);
		if (index >= 0 && envp[index])
			rm_str_arr(envp, envp[index]);
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
	char		**envp;
	const char	**args = (const char **)token->command;

	envp = (char **)shell->env;
	if (!envp || !*(args + 1) || !*envp)
		return (0);
	return (unset_internal(args + 1, envp));
}
