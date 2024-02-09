#include "libft.h"

size_t	get_key_len(const char *s)
{
	size_t	len;
	size_t	key_len;

	key_len = 0;
	len = ft_strlen(s);
	while (key_len < len && s[key_len] != '=')
		key_len++;
	if (s[key_len] == '=' && key_len != 0)
		return (key_len);
	return (-1);
}

int	find_key_env(const char **arr, const char *s, size_t (*f)(const char *s))
{
	size_t	i;
	size_t	key_len;

	if (!arr || !s)
		return (-1);
	i = 0;
	key_len = f(s);
	if (key_len < 0)
		return (-1);
	while (arr[i])
	{
		if (ft_strncmp(arr[i], s, key_len) == 0
			&& arr[i][key_len] && arr[i][key_len] == '=')
				return (i);
		i++;
	}
	return (-1);
}

// get value of key
// use index, then trim off key
// NULL on error or key not found (then caller should not replace)
char	*get_var_val(const char **arr, const char *key)
{
	int		index;
	char	*key_eq;
	char	*val;

	if (!arr || !key)
		return (NULL);
	index = find_key_env(arr, key, ft_strlen);
	if (index != -1 && arr[index])
	{
		key_eq = ft_strjoin(key, "=");
		fprintf(stderr, "key_eq: '%s'\n", key_eq);
		if (!key_eq)
			return (NULL);
		val = ft_substr(arr[index], ft_strlen(key_eq), ft_strlen(arr[index]) - ft_strlen(key_eq));
		fprintf(stderr, "val: %s\n", val);
		free(key_eq);
		if (!val)
			return (NULL);
		return (val);
	}
	return (ft_strdup(""));
}
