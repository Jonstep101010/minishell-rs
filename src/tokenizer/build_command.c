#include "tokens.h"
#include "utils.h"
#include "arr_utils.h"
#include <stdio.h>
#include "libft.h"

char	**get_cmd_arr_token(t_token *token)
{
	int		i;
	char	**cmd_arr;

	i = 0;
	cmd_arr = NULL;
	if (!token || !token->cmd_args)
		return (NULL);
	if (token->cmd_args[0].elem)
	{
		while (token->cmd_args[i].elem)
		{
			if (token->cmd_args[i].type != REDIR)
			{
				cmd_arr = append_str_arr_free(cmd_arr,
					ft_strdup(token->cmd_args[i].elem));
				if (!cmd_arr)
					return (NULL);
			}
			i++;
		}
	}
	return (cmd_arr);
}

// splits into split_pipes, removes whitespace and assigns to tokens
t_token	*get_tokens(char const *trimmed_line)
{
	int		i;
	char	**split_pipes;
	t_token	*token;

	if (!ft_strchr(trimmed_line, '|'))
		split_pipes = append_str_arr(NULL, trimmed_line);
	else
		split_pipes = split_outside_quotes(trimmed_line, "|");
	if (!split_pipes)
		return (eprint("alloc fail!"), NULL);
	if (!*split_pipes)
		return (arr_free(split_pipes), NULL);
	token = init_token(arr_len(split_pipes));
	i = 0;
	while (token && split_pipes[i])
	{
		token[i].split_pipes = ft_strtrim(split_pipes[i], WHITESPACE);
		i++;
	}
	if (!token)
		eprint("alloc fail token");
	i = 0;
	while (token[i].split_pipes && split_pipes[i])
	{
		if (token[i].split_pipes != split_pipes[i])
			free(split_pipes[i]);
		i++;
	}
	free(split_pipes);
	return (token);
}
