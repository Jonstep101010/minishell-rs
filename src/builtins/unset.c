#include "environment.h"
#include "libft.h"
#include "struct.h"
#include "utils.h"
// see if exists, remove using rm_str_arr
// otherwise: error with not found
// implement proper error handling @todo (set codes instead of returning null)
int		unset(char *cmd, char **args, char **envp)
{
	int		index;
	char	*key;

	// key should be at args[1]
	if (!envp)
		return ((void)printf("Error: environment does not exist!\n"), -2);
	if (!args || !*args || arr_len((const char **)args) > 2)
		return ((void)printf("Error: no variable name provided!\n"), -1);
	if (!occurs_exclusively("unset", cmd)
			|| ft_strncmp(cmd, args[0], ft_strlen(cmd)) != 0)
		return ((void)printf("Error: no variable name provided!\n"), -1);
	key = args[1];
	if (!key || !*key)
		return ((void)printf("Error: no variable name provided!\n"), -1);
	index = find_key_env((const char **)envp, key, ft_strlen);
	if (index >= 0 && envp[index])
	{
		printf("removing variable: %s\n", envp[index]);
		rm_str_arr(envp, envp[index]);
		return (0);
	}
	printf("Error: varname does not exist in env!\n");
	return (-1);
}
// int		unset(t_shell *shell, char *key)
// {
// 	int	index;

// 	if (!shell->owned_envp)
// 		return ((void)printf("Error: environment does not exist!\n"), -2);
// 	if (!key || !*key)
// 		return ((void)printf("Error: no variable name provided!\n"), -1);
// 	index = find_key_env((const char **)shell->owned_envp, key, ft_strlen);
// 	if (index >= 0 && shell->owned_envp[index])
// 	{
// 		printf("removing variable: %s\n", shell->owned_envp[index]);
// 		rm_str_arr(shell->owned_envp, shell->owned_envp[index]);
// 		return (0);
// 	}
// 	printf("Error: varname does not exist in env!\n");
// 	return (-1);
// }
