#include "libft.h"
#include "utils.h"
#include "arr_utils.h"
#include <unistd.h>
#include "parser.h"

static int	is_in_set(char c, const char *set)
{
	while (*set)
	{
		if (c == *set)
			return (1);
		set++;
	}
	return (0);
}

static char	**splitter(t_splitter *split, const char *trimmed, const char *set)
{
	while (is_in_set(trimmed[split->i + 1], set))
	{
		split->i++;
		if (split->i >= ft_strlen(trimmed))
			break ;
	}
	if (split->token_end < split->start)
		split->token_end = split->start - 1;
	split->tmp = ft_substr(trimmed, split->start,
			split->token_end - split->start + 1);
	if (!split->tmp)
		return (arr_free(split->arr), NULL);
	split->ret = append_str_arr_free(split->arr, split->tmp);
	if (!split->ret)
		return (arr_free(split->ret), NULL);
	split->arr = split->ret;
	split->start = split->i + 1;
	return (split->ret);
}

// do not touch unless tested changes -> this leaks like a *****
static char	**split_iterator(const char *to_split,
						const char *trim, const char *set)
{
	t_splitter	*split;
	char		**ret;

	split = (t_splitter *)ft_calloc(1, sizeof(t_splitter));
	while (trim[split->i] && split->start < ft_strlen(trim))
	{
		if (!split->quote && (trim[split->i] == '\'' || trim[split->i] == '"'))
			split->quote = trim[split->i];
		else if (split->quote && trim[split->i] == split->quote)
			split->quote = 0;
		if (!split->quote && is_in_set(trim[split->i], set)
			&& !splitter(split, trim, set))
			return (free(split), arr_free(split->ret), NULL);
		split->token_end = split->i;
		split->i++;
	}
	ret = append_str_arr_free(split->ret,
			ft_substr(to_split, split->start, split->i - split->start));
	return (free(split), ret);
}

char	**split_outside_quotes(const char *to_split, const char *set)
{
	char		**ret;
	char		*trimmed;

	trimmed = NULL;
	if (!to_split)
		return (NULL);
	while (is_in_set(*to_split, set))
		to_split++;
	trimmed = ft_strtrim(to_split, set);
	if (!trimmed)
		return (NULL);
	ret = split_iterator(to_split, trimmed, set);
	free(trimmed);
	return (ret);
}
