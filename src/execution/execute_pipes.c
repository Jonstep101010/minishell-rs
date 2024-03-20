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

static void	close_pipe_fds(int **pipes, int token_count)
{
	int	i;

	i = -1;
	while (++i < token_count - 1)
	{
		close(pipes[i][0]);
		close(pipes[i][1]);
	}
}

static void	exec_child(t_shell *shell, int i, int **pipes, int token_count)
{
	if (shell->token[i].has_redir && i == 0)
		heredoc_nopipe(&shell->token[i], shell->env);
	if (i != 0)
		dup2(pipes[i - 1][0], STDIN_FILENO);
	if (i != token_count - 1)
		dup2(pipes[i][1], STDOUT_FILENO);
	if (shell->token[i].has_redir)
		do_redirections(&shell->token[i]);
	close_pipe_fds(pipes, token_count);
	convert_tokens_to_string_array(&shell->token[i]);
	if (!shell->token[i].command)
		return ;
	shell->token[i].cmd_func(shell, &shell->token[i]);
	exit(shell->exit_status);
}

void	execute_pipes(t_shell *shell, int **pipes, int token_count)
{
	int		status;
	int		i;
	pid_t	pid;

	i = -1;
	while (++i < token_count)
	{
		if (shell->token[i].has_redir && i != 0)
			do_heredocs(&shell->token[i], pipes[i - 1], shell->env);
		pid = fork();
		if (pid == -1)
			eprint("fork %s\n", strerror(errno));
		if (pid == 0)
			exec_child(shell, i, pipes, token_count);
	}
	close_pipe_fds(pipes, token_count);
	waitpid(pid, &status, 0);
	while (wait(NULL) > 0)
		;
	if (WIFEXITED(status))
		shell->exit_status = WEXITSTATUS(status);
	i = -1;
	while (++i < token_count - 1)
		free(pipes[i]);
	free(pipes);
}
