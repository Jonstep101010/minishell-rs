/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:31:23 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:31:47 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdio.h>
#include <readline/readline.h>
#include <readline/history.h>
#include <unistd.h>
#include "msh_signals.h"
#include "minishell.h"
#include <stdlib.h>
#include "utils.h"
#include "commands.h"
#include "lexer.h"
#include "struct.h"

void	minishell_loop(t_shell *shell)
{
	char	*trimmed_line;
	char	*readline_line;

	check_signals(&shell->p_termios);
	while (1)
	{
		readline_line = readline("minishell> ");
		if (!readline_line)
			builtin_exit(shell, NULL);
		trimmed_line = get_input(readline_line);
		if (!trimmed_line)
			continue ;
		add_history(trimmed_line);
		if (!*trimmed_line || lexer(shell, trimmed_line) != LEXER_SUCCESS)
			continue ;
		if (shell->env && *shell->env && shell->token)
			execute_commands(shell, shell->token);
	}
}

	// if (ac > 1 || av[1])
	// 	return (printf("do not pass arguments\n"), 1);
int	main(int ac, char **av, char **envp)
{
	t_shell		*shell;

	(void)ac;
	(void)av;
	shell = init_shell(envp);
	if (!shell)
		return (1);
	minishell_loop(shell);
	return (0);
}
