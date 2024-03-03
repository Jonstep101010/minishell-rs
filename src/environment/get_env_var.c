#include <stdlib.h>
#include "libft.h"
#include "environment.h"

// get value of key
// use index, then trim off key
// NULL on error or key not found (then caller should not replace)
char	*get_env_var(char **arr, const char *key)
{
	int			index;
	char		*key_eq;
	char		*val;
	const char	**envp = (const char **)arr;

	if (!arr || !key)
		return (NULL);
	index = find_key_env(envp, key, ft_strlen);
	if (index != -1 && envp[index])
	{
		key_eq = ft_strjoin(key, "=");
		if (!key_eq)
			return (NULL);
		val = ft_substr(envp[index], ft_strlen(key_eq), ft_strlen(arr[index]) - ft_strlen(key_eq));
		free(key_eq);
		if (!val)
			return (NULL);
		return (val);
	}
	return (NULL);
}
