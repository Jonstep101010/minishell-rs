#include "commands.h"
#include "struct.h"
#include "tokens.h"
#include "utils.h"
#include <errno.h>
#include <fcntl.h>
#include <readline/readline.h>
#include <stdbool.h>
#include <string.h>
#include <sys/param.h>
#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include "execution.h"
#include "tokens.h"

static void	exec_last(t_shell *shell, int i, int prevpipe, char **error_elem)
{
	pid_t	cpid;
	int		status;

	cpid = fork ();
	if (cpid == 0)
	{
		if (do_redirections(shell->token[i].cmd_args, error_elem) != 0)
			exit_error(shell, *error_elem);
		dup2(prevpipe, STDIN_FILENO);
		close(prevpipe);
		exit_free(shell, shell->token[i].cmd_func(shell, &shell->token[i]));
	}
	else
	{
		waitpid(cpid, &status, 0);
		close (prevpipe);
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
	int		status;

	pipe(pipefd);
	cpid = fork();
	if (cpid == 0)
	{
		close(pipefd[0]);
		dup2(pipefd[1], STDOUT_FILENO);
		close(pipefd[1]);
		dup2(*prevpipe, STDIN_FILENO);
		close(*prevpipe);
		if (do_redirections(shell->token[i].cmd_args, error_elem) != 0)
			exit_error(shell, *error_elem);
		status = shell->token[i].cmd_func(shell, &shell->token[i]);
		if (shell->env)
			arr_free(shell->env);
		destroy_all_tokens(shell);
		free(shell);
		exit(status);
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
	if (shell->token[i].has_redir && i == token_count - 1)
		do_heredocs(&shell->token[i], &prevpipe, shell->env);
	exec_last(shell, i, prevpipe, &error_elem);
}
