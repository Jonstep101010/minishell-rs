/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   execute_pipes.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:44:46 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:44:47 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "utils.h"
#include "execution.h"
#include "minishell.h"
#include "commands.h"
#include "struct.h"
#include <stdlib.h>
#include <sys/wait.h>
#include <sys/param.h>
#include <unistd.h>
#include "msh_signals.h"

static void	exec_last(t_shell *shell, int i, int *prevpipe, char **error_elem)
{
	pid_t	cpid;
	int		status;

	cpid = fork();
	status = 0;
	if (cpid == 0)
	{
		check_signals_child(&shell->p_termios);
		if (shell->token[i].has_redir)
			do_heredocs(&shell->token[i], prevpipe, shell->env);
		if (do_redirections(shell->token[i].cmd_args, error_elem) != 0)
			exit_error(shell, *error_elem);
		dup2(*prevpipe, STDIN_FILENO);
		close(*prevpipe);
		exit_free(shell, shell->token[i].cmd_func(shell, &shell->token[i]));
	}
	else
	{
		waitpid(cpid, &status, 0);
		close(*prevpipe);
		while (wait(NULL) > 0)
			;
		if (WIFEXITED(status))
			update_exit_status(shell, WEXITSTATUS(status));
	}
}

static void	exec_pipe(t_shell *shell, int i, int *prevpipe, char **error_elem)
{
	int		pipefd[2];
	pid_t	cpid;

	pipe(pipefd);
	cpid = fork();
	if (cpid == 0)
	{
		check_signals_child(&shell->p_termios);
		close(pipefd[0]);
		dup2(pipefd[1], STDOUT_FILENO);
		close(pipefd[1]);
		dup2(*prevpipe, STDIN_FILENO);
		close(*prevpipe);
		if (do_redirections(shell->token[i].cmd_args, error_elem) != 0)
			exit_error(shell, *error_elem);
		exit_free(shell, shell->token[i].cmd_func(shell, &shell->token[i]));
	}
	else
	{
		close(pipefd[1]);
		close(*prevpipe);
		*prevpipe = pipefd[0];
	}
}

void	execute_pipes(t_shell *shell, int token_count)
{
	int		i;
	int		prevpipe;
	char	*error_elem;

	i = -1;
	prevpipe = dup(STDIN_FILENO);
	while (++i < token_count - 1)
	{
		if (shell->token[i].has_redir && i != token_count - 1)
			do_heredocs(&shell->token[i], &prevpipe, shell->env);
		exec_pipe(shell, i, &prevpipe, &error_elem);
	}
	exec_last(shell, i, &prevpipe, &error_elem);
}
