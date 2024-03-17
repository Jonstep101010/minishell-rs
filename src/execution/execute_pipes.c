#include "commands.h"
#include "struct.h"
#include "tokens.h"
#include "utils.h"
#include <errno.h>
#include <stdbool.h>
#include <string.h>
#include <sys/param.h>
#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include "libft.h"
#include "execution.h"

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
	// do redirections
	if (i != 0)
		dup2(pipes[i - 1][0], STDIN_FILENO);
	if (i != token_count - 1)
		dup2(pipes[i][1], STDOUT_FILENO);
	close_pipe_fds(pipes, token_count);
	convert_tokens_to_string_array(&shell->token[i]);
	if (!shell->token[i].command)
		return ;
	if (shell->token[i].cmd_func != not_builtin)
	{
		if (shell->token[i].cmd_func == builtin_exit)
			shell->exit_status = shell->token[i].cmd_func(shell, &shell->token[i]);
		else
			shell->token[i].cmd_func(shell, &shell->token[i]);
		exit(shell->exit_status);
	}
	else if (execvp(shell->token[i].cmd_args[0].elem,
			shell->token[i].command) == -1)
		execve_fail(shell, shell->token[i].cmd_args[0].elem);
}

void	execute_pipes(t_shell *shell, int **pipes, int token_count)
{
	int		status;
	int		i;
	pid_t	pid;

	i = -1;
	while (++i < token_count)
	{
		// do heredocs
		pid = fork();
		if (pid == -1)
			eprint("fork %s\n", strerror(errno));
		if (pid == 0)
			exec_child(shell, i, pipes, token_count);
	}
	close_pipe_fds(pipes, token_count);
	waitpid(pid, &status, 0);
	if (WIFEXITED(status))
		shell->exit_status = WEXITSTATUS(status);
	i = -1;
	while (++i < token_count - 1)
		free(pipes[i]);
	free(pipes);
}
