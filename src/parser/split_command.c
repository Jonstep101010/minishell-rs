#include "parser.h"
#include "libft.h"
#include "struct.h"
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

static void	init_splitter(t_splitter *split, char *to_split)
{
	split->quote = 0;
	split->i = 0;
	split->start = 0;
	split->len = ft_strlen(to_split);
	split->ret = NULL;
}

static char	**split_iterator(t_splitter *split, char *to_split, char c)
{
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
				return (NULL);
			split->ret = append_str_arr(split->ret, split->tmp);
			free(split->tmp);
			if (!split->ret)
				return (NULL);
			split->start = split->i + 1;
		}
		split->i++;
	}
	return (split->ret);
}

char	**split_outside_quotes(char *to_split, char c)
{
	t_splitter	split;

	if (!to_split)
		return (NULL);
	init_splitter(&split, to_split);
	if (!split_iterator(&split, to_split, c))
		return (NULL);
	split.tmp = ft_substr(to_split,
		split.start, split.i - split.start);
	ft_printf("%s\n", split.tmp);
	split.ret = append_str_arr(split.ret, split.tmp);
	if (!split.ret)
		return (NULL);
	free(split.tmp);
	return (split.ret);
}

char	**split_command(t_shell *shell)
{
	shell->tokens = split_outside_quotes(shell->line, '|');
	if (!shell->tokens)
		return (NULL);
	print_arr(shell->tokens);
	// @todo this would usually take place in the childs
	shell->command = split_outside_quotes(
		shell->tokens[0], ' ');
	print_arr_sep(shell->command, '{', '}');
	arr_free(shell->tokens);
	return (shell->tokens);
}
