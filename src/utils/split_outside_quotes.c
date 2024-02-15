#include "libft.h"
#include "utils.h"
#include <unistd.h>
#include "parser.h"

// takes literal pipes not in quotes
// splits into strings
// split since last pipe or start of string
// traverse to check
// split pipes and spaces outside of quotes (in childs)
// check for pipes outside quotes
// (childs will handle their own splitting)

// check for spaces if no pipes, otherwise childs handle it
static void	init_splitter(t_splitter *split, size_t to_split_len)
{
	ft_bzero(split, sizeof(t_splitter));
	split->len = to_split_len;
}

// this is beyond cursed
static	char	**return_check(char **ret)
{
	if (!ret)
		return ((char **) ft_calloc(1, sizeof(char *)));
	return (ret);
}

static int is_in_set(char c, const char *set)
{
	while (*set)
	{
		if (c == *set)
			return (1);
		set++;
	}
	return (0);
}

// do not touch unless tested changes -> this leaks like a *****
static char	**split_iterator(
		t_splitter *split, const char *to_split, const char *set)
{
	while (to_split[split->i] && split->start < split->len)
	{
		if (split->quote == 0 &&
			(to_split[split->i] == '\'' || to_split[split->i] == '"'))
				split->quote = to_split[split->i];
		else if (split->quote != 0 && to_split[split->i] == split->quote)
			split->quote = 0;
		else if (split->quote == 0 && is_in_set(to_split[split->i], set))
		{
			while (is_in_set(to_split[split->i + 1], set))
			{
				split->i++;
				if (split->i >= split->len)
					break;
			}
			if (split->token_end < split->start)
				split->token_end = split->start - 1;
			split->tmp = ft_substr(to_split, split->start,
					split->token_end - split->start + 1);
			if (!split->tmp)
				return (arr_free(split->arr), NULL);
			split->ret = append_str_arr((const char **)split->arr, split->tmp);
			free(split->tmp);
			arr_free(split->arr);
			if (!split->ret)
				return (arr_free(split->ret), NULL);
			split->arr = split->ret;
			split->start = split->i + 1;
		}
		else
			split->token_end = split->i;
		split->i++;
	}
	return (return_check(split->ret));
}

char	**split_outside_quotes(const char *to_split, const char *set)
{
	t_splitter	split;
	char		**ret;
	char		*last_token;

	if (!to_split)
		return (NULL);
	init_splitter(&split, ft_strlen(to_split));
	if (is_in_set(to_split[0], set) || is_in_set(to_split[ft_strlen(to_split) - 1], set))
	{
		while (is_in_set(*to_split, set))
			to_split++;
		if (!*to_split)
			return (arr_dup((const char *[]){"", NULL}));
		split.tmp = ft_strdup(to_split);
		if (!split.tmp)
			return (NULL);
		split.trim = ft_strlen(split.tmp);
		while (split.trim > 0 && is_in_set(split.tmp[split.trim - 1], set))
			split.trim--;
		if (split.trim == 0)
			return (ft_calloc(1, sizeof(char *)));
		split.tmp2 = ft_substr(split.tmp, 0, split.trim);
		free(split.tmp);
		if (!split.tmp2)
			return (NULL);
		split.not_last_token = split_iterator(&split, split.tmp2, set);
		free(split.tmp2);
	}
	else
		split.not_last_token = split_iterator(&split, to_split, set);
	if (!split.not_last_token)
		return (arr_free(split.arr), NULL);
	last_token = ft_substr(to_split, split.start, split.i - split.start);
	ret = append_str_arr((const char **)split.not_last_token, last_token);
	free(last_token);
	arr_free(split.not_last_token);
	return (ret);
}
