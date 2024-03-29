/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 19:51:58 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 19:51:59 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "lexer.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <stdlib.h>
#include <unistd.h>
#include "minishell.h"
#include "tokens.h"
#include <sys/wait.h>

t_lexer	*lexer_checks_basic(const char *s);

int	lexer(t_shell *shell, const char *trimmed_line)
{
	t_lexer	*lex;

	if (!*trimmed_line)
	{
		get_input(NULL);
		return (0);
	}
	lex = lexer_checks_basic(trimmed_line);
	if (!lex->result)
	{
		update_exit_status(shell, lex->lexer);
		get_input(NULL);
		free(lex);
		return (1);
	}
	free(lex);
	shell->token = tokenize(shell, trimmed_line);
	get_input(NULL);
	if (!shell->token)
		return (-1);
	if (!shell->token->cmd_args)
		return (destroy_all_tokens(shell), -1);
	return (LEXER_SUCCESS);
}
