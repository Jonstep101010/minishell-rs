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

static size_t	get_len_until(const char *s, char c)
{
	size_t	i;

	i = 0;
	if (!s)
		return (0);
	while (s[i] && s[i] != c)
		i++;
	return (i);
}

char	*free_both_join(char *s1, char *s2);

/**
 * @brief replace expandable variable with its value
 * @details check for valid key & null before calling, do not hand in without single $ at beginning, key does not have to exist, only single variable will be expanded
 *
 * @param input $KEYsomething
 * @param envp {"KEY=VALUE", NULL}
 * @return char* VALUEsomething
 */
char	*expand_var(const char *input, const char **envp)
{
	size_t	i;
	char	*tmp;
	char	*remainder;
	char	*val;

	if (!input || !envp || !*envp)
		return (NULL);
	if (*input != '$')
		return (ft_strdup(input));
	i = get_len_until(&input[1], '$') + 2;
	while (i > 2 && --i)
	{
		tmp = ft_substr(input, 1, i - 1);
		val = get_env_var((char **)envp, tmp);
		free(tmp);
		if (val)
		{
			remainder = ft_substr(input, i, ft_strlen(input));
			return (free_both_join(val, remainder));
		}
	}
	return (ft_strdup(""));
}
