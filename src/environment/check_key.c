#include "libft.h"
#include <stdbool.h>

bool	check_valid_key(const char *s)
{
	int	i;

	i = 0;
	if (*s == '=')
		return (false);
	while (s[i] && s[i] != '=')
	{
		if (ft_isalnum(s[i]) || s[i] == '_')
			i++;
		else
			return (false);
	}
	if (s[i] == '=' && s[i + 1] == '=')
		return (false);
	return (true);
}
