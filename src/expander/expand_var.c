#include <stdlib.h>
#include "libft.h"

char	*get_env_var(char *const *env, const char *key);

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
char	*expand_var(char const *input, char *const *envp)
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
