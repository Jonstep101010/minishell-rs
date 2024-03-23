#include "libft.h"
#include <stddef.h>
#include <stdbool.h>
#include "str_utils.h"

/**
 * only keeps if nested and not same as outer
 */
void	*do_quote_bs(const char *s, int *quote)
{
	char	*tmp;
	size_t	len;
	size_t	tmp_len;

	len = ft_strlen(s);
	tmp = (char *)ft_calloc(len + 1, sizeof(char));
	if (!tmp)
		return (NULL);
	while (*s)
	{
		if (*quote == 0 && (*s == '\'' || *s == '"'))
			*quote = *s;
		else if (*quote != 0 && *s == *quote)
			*quote = 0;
		else
		{
			tmp_len = ft_strlen(tmp);
			tmp[tmp_len] = *s;
			tmp[tmp_len + 1] = '\0';
		}
		s++;
	}
	return (tmp);
}

bool	interpret_quotes(char **cmd_arr)
{
	int		i;
	int		quote;
	char	*tmp;

	i = 0;
	quote = 0;
	if (!cmd_arr)
		return (true);
	while (cmd_arr[i])
	{
		if (str_cchr(cmd_arr[i], '\'') != 0
			|| str_cchr(cmd_arr[i], '"') != 0)
		{
			tmp = do_quote_bs(cmd_arr[i], &quote);
			if (!tmp)
				return (false);
			free(cmd_arr[i]);
			cmd_arr[i] = tmp;
		}
		i++;
	}
	return (true);
}
