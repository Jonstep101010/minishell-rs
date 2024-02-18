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

size_t	get_len_until(const char *s, char c)
{
	size_t	i;

	i = 0;
	if (!s)
		return (0);
	while (s[i] && s[i] != c)
		i++;
	return (i);
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
		// fprintf(stderr, "key_eq: '%s'\n", key_eq);
		if (!key_eq)
			return (NULL);
		val = ft_substr(arr[index], ft_strlen(key_eq), ft_strlen(arr[index]) - ft_strlen(key_eq));
		// fprintf(stderr, "val: %s\n", val);
		free(key_eq);
		if (!val)
			return (NULL);
		return (val);
	}
	return (NULL);
}

char	*join_strings_free(int count, ...);

/**
 * @brief replace expandable variable with its value
 * @details check for valid key & null before calling, do not hand in without single $ at beginning, key does not have to exist, only single variable will be expanded
 *
 * @param input $KEYsomething
 * @param envp {"KEY=VALUE", NULL}
 * @return char* VALUEsomething
 */
char	*expand_in_string(const char *input, const char **envp)
{
	size_t	i;
	char	*tmp;
	char	*remainder;
	char	*val;

	if (!input || !envp || !*envp)
		return (NULL);
	if (*input != '$')
		return (ft_strdup(input));
	i = get_len_until(&input[1], '$') + 1;
	// fprintf(stderr, "i: %zu\n", i);
	// for (size_t j = 0; j < i; j++)
		// fprintf(stderr, "%c", input[j]);
	// fprintf(stderr, "\n");
	while (i > 0)
	{
		tmp = ft_substr(input, 1, i - 1);
		// fprintf(stderr, "tmp: %s\n", tmp);
		fprintf(stderr, "input: %s\n", &input[1]);
		val = get_var_val(envp, tmp);
		// fprintf(stderr, "val: %s\n", val);
		free(tmp);
		if (val)
		{
			remainder = ft_substr(input, i, ft_strlen(input));
			// fprintf(stderr, "joined: '%s'\n", join_strings_free(2, val, remainder));
			return (join_strings_free(2, val, remainder));
		}
		i--;
	}
	return (ft_strdup(""));
}
