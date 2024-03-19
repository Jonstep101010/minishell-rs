#include "commands.h"
#include "environment.h"
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
#include "libft.h"
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

void	do_redirections(t_token *token);

static void	do_heredocs(t_token *token, const int *target, char **env)
{
	int		fd;
	int		i;
	char	*line;

	line = NULL;
	char	*expanded;
	i = -1;
	expanded = NULL;
	while (token->cmd_args[++i].elem)
	{
		if (token->cmd_args[i].redir == HEREDOC)
		{
			eprint("token[%d] has heredoc!", i);
			fd = open(token->cmd_args[i].elem, O_RDWR | O_CREAT | O_TRUNC, 0644);
			if (fd == -1)
				return (eprint("%s", strerror(errno)), exit(1));
			while (1)
			{
				line = readline("> ");
				if (equal(token->cmd_args[i].elem, line) || !line)
					break ;
				if (ft_strchr(line, '$'))
				{
					expanded = expander(line, env);
					if (expanded && !equal(expanded, line))
						ft_putendl_fd(expanded, fd);
					free(expanded);
				}
				else
					ft_putendl_fd(line, fd);
				free(line);
			}
			free(line);
			close(fd);
			fd = open(token->cmd_args[i].elem, O_RDONLY);
			dup2(fd, *target);
			close(fd);
		}
	}
}

static void	exec_child(t_shell *shell, int i, int **pipes, int token_count)
{
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

void	do_redirections(t_token *token)
{
	int		fd;
	int		i;

	i = 0;
	while (token->cmd_args[i].elem)
	{
		if (token->cmd_args[i].type == REDIR && token->cmd_args[i].redir != HEREDOC)
		{
			eprint("redir elem: %s\n", token->cmd_args[i].elem);
			fd = -1;
			if (token->cmd_args[i].redir == INPUT_REDIR)
				fd = open(token->cmd_args[i].elem, O_RDONLY);
			else if (token->cmd_args[i].redir == OUTPUT_REDIR)
			{
				fd = open(token->cmd_args[i].elem, O_WRONLY | O_CREAT | O_TRUNC, 0644);
			}
			else if (token->cmd_args[i].redir == APPEND)
				fd = open(token->cmd_args[i].elem, O_WRONLY | O_CREAT | O_APPEND, 0644);
			if (fd == -1)
				eprint("open %s\n", strerror(errno));
			else if (token->cmd_args[i].redir != INPUT_REDIR)
				dup2(fd, STDOUT_FILENO);
			else
				dup2(fd, STDIN_FILENO);
			close(fd);
		}
		i++;
	}
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
