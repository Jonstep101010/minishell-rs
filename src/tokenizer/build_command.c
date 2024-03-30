/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   build_command.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:03:57 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/30 11:25:51 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "tokens.h"
#include "utils.h"
#include "arr_utils.h"
#include <stdio.h>
#include "libft.h"
#include "parser.h"

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

	split_pipes = split_outside_quotes(trimmed_line, "|");
	if (!split_pipes)
		return (eprint("alloc fail!"), NULL);
	if (!*split_pipes)
		return (arr_free(split_pipes), NULL);
	token = init_token(arr_len(split_pipes));
	if (!token)
		eprint("alloc fail token");
	i = 0;
	while (token && split_pipes[i])
	{
		token[i].split_pipes = split_pipes[i];
		i++;
	}
	free(split_pipes);
	return (token);
}
