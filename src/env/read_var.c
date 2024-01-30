#include "env.h"
#include "libft.h"
#include <stdio.h>

// get value of key
// use index, then trim off key
// NULL on error or key not found (then caller should not replace)
char	*get_var_val(char **arr, const char *key)
{
	int		index;
	char	*key_eq;
	char	*val;

	if (!arr || !key)
		return (NULL);
	index = find_key_env(arr, key, ft_strlen);
	if (index != -1 && arr[index])
	{
		// fprintf(stderr, "arr[index]: %s\n", arr[index]);
		key_eq = ft_strjoin(key, "=");
		fprintf(stderr, "key_eq: '%s'\n", key_eq);
		if (!key_eq)
			return (NULL);
		// val = ft_strdup(&arr[index][ft_strlen(key_eq)]);
		val = ft_substr(arr[index], ft_strlen(key_eq), ft_strlen(arr[index]) - ft_strlen(key_eq));
		fprintf(stderr, "val: %s\n", val);
		free_null(key_eq);
		if (!val)
			return (NULL);
		return (val);
	}
	return (ft_strdup(""));
}
