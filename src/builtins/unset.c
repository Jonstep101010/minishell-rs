#include "environment.h"
#include "libft.h"
#include "struct.h"
#include "utils.h"
// see if exists, remove using rm_str_arr
// otherwise: error with not found
// implement proper error handling @todo (set codes instead of returning null)
char	**unset(t_shell *shell, char *key)
{
	if (!shell->owned_envp || !key || !*key)
		return (NULL);
	int	index = find_key_env((const char **)shell->owned_envp, key, ft_strlen);
	// printf("index: %d\n", index);
	// printf("key: '%s'\n", key);
	if (index == -1)
	{
		ft_printf("Error: varname does not exist in env!\n");
		return (NULL);
	}
	if (index >= 0 && shell->owned_envp[index])
	{
		ft_printf("removing variable: %s\n", shell->owned_envp[index]);
		rm_str_arr(shell->owned_envp, shell->owned_envp[index]);
	}
	return (shell->owned_envp);
}
