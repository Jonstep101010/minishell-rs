#include "libft.h"
#include <stdbool.h>

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

int	find_key_env(char *const *env, char const *s, size_t (*f)(char const *s))
{
	size_t	i;
	size_t	key_len;

	if (!env || !s)
		return (-1);
	i = 0;
	key_len = f(s);
	if (key_len < 0)
		return (-1);
	while (env[i])
	{
		if (ft_strncmp(env[i], s, key_len) == 0
			&& env[i][key_len] && env[i][key_len] == '=')
				return (i);
		i++;
	}
	return (-1);
}

bool	check_valid_key(const char *s)
{
	int	i;

	i = 0;
	while (s[i] && s[i] != '=')
	{
		if (ft_isalnum(s[i]) || s[i] == '_')
			i++;
		else
			return (false);
	}
	return (true);
}
