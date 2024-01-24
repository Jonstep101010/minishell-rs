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

int	find_key_env(char **arr, const char *s, size_t (*f)(const char *s))
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
		if (ft_strncmp(arr[i], s, key_len - 1) == 0
			&& arr[i][key_len] && arr[i][key_len] == '=')
				return (i);
		i++;
	}
	return (-1);
}
