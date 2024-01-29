#include "ft_printf.h"
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
char	**split_outside_quotes(char *to_split, char c)
{
	int		quote;
	size_t	i;
	size_t	start;
	size_t	len;
	char	**ret;
	char	*tmp;

	quote = 0;
	i = 0;
	start = 0;
	if (!to_split)
		return (NULL);
	len = ft_strlen(to_split);
	ret = NULL;
	while (to_split[i] && start < len)
	{
		if (quote == 0 && (to_split[i] == '\'' || to_split[i] == '"'))
			quote = to_split[i];
		else if (quote != 0 && to_split[i] == quote)
			quote = 0;
		else if (quote == 0 && to_split[i] == c)
		{
			tmp = ft_substr(to_split, start, i - start);
			ft_printf("%s\n", tmp);
			ret = append_str_arr(ret, tmp);
			if (!ret || !tmp)
				return (NULL);
			free(tmp);
			start = i + 1;
		}
		i++;
	}
	tmp = ft_substr(to_split, start, i - (start));
	ft_printf("%s\n", tmp);
	ret = append_str_arr(ret, tmp);
	if (!ret)
		return (NULL);
	free(tmp);
	return (ret);
}

char	**split_command(t_shell *shell)
{
	shell->tokens = split_outside_quotes(shell->line, '|');
	// exit(0);
	if (!shell->tokens)
		return (NULL);
	print_arr(shell->tokens);
	// @todo this would usually take place in the childs
	shell->command = split_outside_quotes(
		shell->tokens[2], ' ');
	print_arr_sep(shell->command, '{', '}');
	arr_free(shell->tokens);
	return (shell->tokens);
}
