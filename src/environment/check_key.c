#include <stdbool.h>
#include "libft.h"

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
