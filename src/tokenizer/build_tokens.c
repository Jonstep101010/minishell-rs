/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   build_tokens.c                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:04:23 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:06:45 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stddef.h>
#include <sys/param.h>
#include "arr_utils.h"
#include "commands.h"
#include "libft.h"
#include "tokens.h"
#include "utils.h"
#include "libutils.h"
#include "environment.h"
#include "struct.h"
#include "parser.h"

void			rm_prefix_redir_word(t_arg *arg);
void			parse_redir_types(t_arg *arg);
enum e_redir	check_redirections(t_arg *cmd_args);

static void	*expand_if_allowed(t_token *token, size_t ii, char *const *env)
{
	char	*tmp;

	if (token->cmd_func != builtin_env
		&& str_cchr(token->cmd_args[ii].elem, '$') != 0)
	{
		tmp = expander(token->cmd_args[ii].elem, env);
		if (!tmp)
			return (NULL);
		if (ft_strncmp(tmp, token->cmd_args[ii].elem, MAX(ft_strlen(tmp),
					ft_strlen(token->cmd_args[ii].elem)) == 0))
			free(tmp);
		else
		{
			free(token->cmd_args[ii].elem);
			token->cmd_args[ii].elem = tmp;
		}
	}
	return (token);
}

static void	*setup_token(t_token *token, char *const *env)
{
	size_t	ii;

	if (!token || !token->split_pipes)
		return (NULL);
	token->tmp_arr = split_outside_quotes(token->split_pipes, WHITESPACE);
	if (!token->tmp_arr)
		return (free_null(&token->split_pipes), NULL);
	token->cmd_args = init_cmdargs(arr_len(token->tmp_arr));
	if (!token->cmd_args)
		return (arr_free(token->tmp_arr), NULL);
	ii = 0;
	while (token->tmp_arr[ii])
	{
		token->cmd_args[ii].elem = token->tmp_arr[ii];
		if (!expand_if_allowed(token, ii, env))
			return (NULL);
		ii++;
	}
	free_null(&(token->tmp_arr));
	return (token);
}

static void	rm_quotes(t_arg *cmd_arg)
{
	char	*tmp;
	int		quote;
	int		i;

	quote = 0;
	i = -1;
	while (cmd_arg[++i].elem)
	{
		tmp = do_quote_bs(cmd_arg[i].elem, &quote);
		if (!tmp)
			return ;
		free_null(&cmd_arg[i].elem);
		cmd_arg[i].elem = tmp;
	}
}

static void	*inner_loop(t_token *token)
{
	int	i;

	if (check_redirections(token->cmd_args))
	{
		token->has_redir = true;
		parse_redir_types(token->cmd_args);
		rm_prefix_redir_word(token->cmd_args);
	}
	i = 0;
	while (token->cmd_args[i].elem)
	{
		if (token->cmd_args[i].type == REDIR)
			i++;
		else
			break ;
	}
	set_cmd_func(token->cmd_args[i].elem, token);
	rm_quotes(token->cmd_args);
	return (token);
}

void	*tokenize(t_shell *shell, char const *trimmed_line)
{
	size_t	i;

	i = 0;
	shell->token = get_tokens(trimmed_line);
	if (!shell->token)
		return (NULL);
	while (shell->token[i].split_pipes)
	{
		if (!setup_token(&shell->token[i], shell->env))
			return (destroy_all_tokens(shell), NULL);
		inner_loop(&shell->token[i]);
		i++;
	}
	return (shell->token);
}
