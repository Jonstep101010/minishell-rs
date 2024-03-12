#include "expander.h"
#include "libft.h"
#include "utils.h"
#include "libutils.h"
#include "environment.h"

static int	check_index_advance(const char *s, int i)
{
	int	count;

	count = 0;
	while (s[i + count] && s[i + count + 1]
		&& !ft_strchr("$\"'/? )(", s[i + count + 1]))
		count++;
	if (s[i + count] && s[i + count + 1] == '?')
		count += 1;
	return (count);
}

static char *expand_inside(char *key, char *const *env, int *i)
{
	const size_t	len = ft_strlen(key);
	char			*save;

	save = NULL;
	if (!*key)
	{
		save = append_char_str(NULL, '$');// handle echo "$ ", echo $"42$"
		if (!save)
			return (free(key), NULL);
	}
	else if (get_index_env(env, key) != -1)
		save = get_env(env, key);
	if (!save)
		save = ft_strdup("");
	free(key);
	if (!save)
		return (NULL);
	*i += len;
	return (save);
}

static void	check_quotes(const char *s, bool *expand, int *double_quote)
{
	if (*s == '"' && *double_quote == 0)
		*double_quote = 1;
	else if (*s == '"' && *double_quote == 1)
		*double_quote = 0;
	if (*s == '\'' && *double_quote == 0
		&& s[check_index_advance(s, 0)] != '"')
		*expand = !*expand;
}

// this gets passed an individual element of the input string
// (already split by pipes and whitespace)
// this runs until the end of the string, expanding variables as it goes
// account for edge cases such as "'$VAR'" ('VAL') "'"'$VAR'"'" ('$VAR')
char	*expand(char const *s, char *const *env)
{
	int		i;
	bool	expand;
	int		double_quote;
	char	*ret;
	char	*key;

	i = -1;
	expand = true;
	double_quote = 0;
	ret = ft_strdup("");
	while (s[++i])
	{
		check_quotes(&s[i], &expand, &double_quote);
		if (expand && (s[i] == '$'
			&& s[i + 1]	&& !ft_strchr("$()", s[i + 1])))
		{
			key = ft_substr(s, i + 1, check_index_advance(s, i));
			if (!key)
				return (free(ret), eprint("alloc fail!"), NULL);
	 		ret = free_both_join(ret, expand_inside(key, env, &i));
		}
		else
			ret = append_char_str(ret, s[i]);
	}
	return (ret);
}
