#include "parser.h"
#include "libft.h"
#include "utils.h"
#include <unistd.h>

// @follow-up will be called by children if pipes present

// takes literal pipes not in quotes
// splits into strings
// split since last pipe or start of string
// traverse to check
// split pipes and spaces outside of quotes (in childs)
// check for pipes outside quotes
// (childs will handle their own splitting)

// check for spaces if no pipes, otherwise childs handle it

static char	**split_iterator(t_splitter *split, char **new_arr, const char *to_split, char c)
{
	split->quote = 0;
	split->i = 0;
	split->start = 0;
	split->len = ft_strlen(to_split);
	split->ret = NULL;
	split->tmp = NULL;
	if (!new_arr)
		return (NULL);
	while (to_split[split->i] && split->start < split->len)
	{
		if (split->quote == 0 &&
			(to_split[split->i] == '\'' || to_split[split->i] == '"'))
				split->quote = to_split[split->i];
		else if (split->quote != 0 && to_split[split->i] == split->quote)
			split->quote = 0;
		else if (split->quote == 0 && to_split[split->i] == c)
		{
			split->tmp = ft_substr(to_split, split->start,
					split->i - split->start);
			if (!split->tmp)
				return (arr_free(new_arr), NULL);
			split->ret = append_str_arr((const char **)new_arr, split->tmp);
			free(split->tmp);
			if (!split->ret)
				return (arr_free((char **)split->ret), arr_free(new_arr), NULL);
			split->start = split->i + 1;
		}
		split->i++;
	}
	return (split->ret);
}

char	**split_outside_quotes(const char *to_split, char c)
{
	t_splitter	split;
	char		**ret;
	char		**notlast;
	char		**new_arr;
	char		*substr;

	if (!to_split)
		return (NULL);
	new_arr = (char **) ft_calloc(1, sizeof(char *));
	if (!new_arr)
		return (NULL);
	notlast = split_iterator(&split, new_arr, to_split, c);
	if (!notlast)
		return (NULL);
	substr = ft_substr(to_split, split.start, split.i - split.start);
	ft_printf("%s\n", substr);
	ret = append_str_arr((const char **)notlast, substr);
	free(substr);
	arr_free(notlast);
	return (ret);
}
