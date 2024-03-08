#include <stdlib.h>
#include "libft.h"
#include "environment.h"

// get value of key
// use index, then trim off key
// NULL on error or key not found (then caller should not replace)
char	*get_env_var(char *const *env, char const *key)
{
	int			index;
	char		*key_eq;
	char		*val;

	if (!env || !key)
		return (NULL);
	index = find_key_env(env, key, ft_strlen);
	if (index != -1 && env[index])
	{
		key_eq = ft_strjoin(key, "=");
		if (!key_eq)
			return (NULL);
		val = ft_substr(env[index], ft_strlen(key_eq), ft_strlen(env[index]) - ft_strlen(key_eq));
		free(key_eq);
		if (!val)
			return (NULL);
		return (val);
	}
	return (NULL);
}
