#include "environment.h"
#include "libft.h"
#include "utils.h"
// see if exists, remove using rm_str_arr
// otherwise: error with not found
// implement proper error handling @todo (set codes instead of returning null)
char	**unset(char **envp, char *key)
{
	if (!envp || !key[1])
		return (NULL);
	if (arr_len((const char **)key) > 2)
		return (ft_printf("Error: too many arguments!\n"), NULL);
	int	index = find_key_env((const char **)envp, key, ft_strlen);
	if (index == -1)
	{
		ft_printf("Error: varname does not exist in env!\n");
		return (NULL);
	}
	if (index >= 0 && envp[index])
	{
		ft_printf("removing variable: %s\n", envp[index]);
		rm_str_arr(envp, envp[index]);
	}
	return (envp);
}
