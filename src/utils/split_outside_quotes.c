#include "libft.h"
#include "utils.h"
#include <unistd.h>

typedef struct s_splitter
{
	int		quote;
	size_t	i;
	size_t	start;
	size_t	len;
	char	**ret;
	char	*tmp;
	char	**arr;
}	t_splitter;

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
	split->quote = 0;
	split->i = 0;
	split->start = 0;
	split->len = to_split_len;
	split->ret = NULL;
	split->tmp = NULL;
}

// this is beyond cursed
static	char	**return_check(char **ret)
{
	if (!ret)
		return ((char **) ft_calloc(1, sizeof(char *)));
	return (ret);
}

// do not touch unless tested changes -> this leaks like a *****
static char	**split_iterator(
		t_splitter *split, const char *to_split, char c)
{
	init_splitter(split, ft_strlen(to_split));
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
				return (arr_free(split->arr), NULL);
			split->ret = append_str_arr((const char **)split->arr, split->tmp);
			free(split->tmp);
			arr_free(split->arr);
			if (!split->ret)
				return (arr_free(split->ret), NULL);
			split->arr = split->ret;
			split->start = split->i + 1;
		}
		split->i++;
	}
	return (return_check(split->ret));
}

char	**split_outside_quotes(const char *to_split, char c)
{
	t_splitter	split;
	char		**ret;
	char		**not_last_token;
	char		*last_token;

	if (!to_split)
		return (NULL);
	split.arr = NULL;
	not_last_token = split_iterator(&split, to_split, c);
	if (!not_last_token)
		return (arr_free(split.arr), NULL);
	last_token = ft_substr(to_split, split.start, split.i - split.start);
	ret = append_str_arr((const char **)not_last_token, last_token);
	free(last_token);
	arr_free(not_last_token);
	return (ret);
}
