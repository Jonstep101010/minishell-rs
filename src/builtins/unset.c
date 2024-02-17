#include "environment.h"
#include "libft.h"
#include "utils.h"

/**
 * @brief lookup varname if it is valid, remove it from envp
 * @audit dprintf replace (forbidden)
 * @param args to read
 * @param envp to modify
 * @return 1 on invalid varname, 0 on success
 */
int		unset(const char **args, char **envp)
{
	int		index;
	size_t	i;

	if (!envp || !*(args + 1) || !*envp)
		return (0);
	i = 1;
	while (args[i] && *args[i])
	{
		if (!check_valid_key(args[i]))
			return (dprintf(STDERR_FILENO, "minishell: unset: '%s': invalid variable name\n", args[i]), 1);
		index = find_key_env((const char **)envp, args[i], ft_strlen);
		if (index >= 0 && envp[index])
			rm_str_arr(envp, envp[index]);
		i++;
	}
	return (0);
}
