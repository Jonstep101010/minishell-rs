#include "commands.h"
#include "libft.h"
#include "struct.h"
#include "tokens.h"
#include "utils.h"
#include <errno.h>
#include <stdbool.h>
#include <string.h>
#include <sys/param.h>
#include <sys/wait.h>
#include <unistd.h>
#include "execution.h"

/**
 * @brief
 * @audit atm only works for execution of one command (forbidden function)
 * @audit should be renamed
 * @param shell
 * @param token pointer to element in array of tokens
 */
int	not_builtin(t_shell *shell, t_token *token)
{
	// implement signals, redirs
	// find path_to_bin, check for permissions (check for not found)... access, execve fail
	if (execvp(token->cmd_args[0].elem, token->command) == -1)
		execve_fail(shell, token->cmd_args[0].elem);
	exit(127);
}

static int	**create_pipes(int token_count)
{
	int	**pipes;
	int	i;

	i = -1;
	pipes = ft_calloc(token_count - 1, sizeof(int *));
	if (!pipes)
		return (NULL);
	while (++i < token_count - 1)
	{
		pipes[i] = ft_calloc(2, sizeof(int));
		if (!pipes[i])
		{
			while (--i >= 0)
				free(pipes[i]);
			free(pipes);
			return (NULL);
		}
		pipe(pipes[i]);
	}
	return (pipes);
}

size_t	arr_len_size(void *arr, size_t size);

static bool forkable_builtin(t_token *token)
{
	return (token->cmd_func != builtin_exit && token->cmd_func != builtin_env
		&& token->cmd_func != builtin_export && token->cmd_func != builtin_unset
		&& token->cmd_func != builtin_cd);
}

static void	exec_single_command(t_shell *shell, t_token *token)
{
	pid_t	pid;
	int		status;

	eprint("exec_single_command");
	if (!forkable_builtin(token))
	{
		eprint("not forkable builtin");
		do_redirections(token);
		convert_tokens_to_string_array(token);
		token->cmd_func(shell, token);
	}
	else
	{
		eprint("forkable function");
		pid = fork();
		if (pid < 0)
			return (eprint("fork %s", strerror(errno)));
		if (pid == 0)
		{
			if (token->has_redir)
				heredoc_nopipe(token, shell->env);
			do_redirections(token);
			convert_tokens_to_string_array(token);
			if (!token->command)
				exit(-1);
			token->cmd_func(shell, token);
			exit(shell->exit_status);
		}
		else
		{
			waitpid(pid, &status, 0);
			if (WIFEXITED(status))
				shell->exit_status = WEXITSTATUS(status);
		}
	}
}

void	execute_commands(t_shell *shell, t_token *token)
{
	int	token_count;
	int	**pipes;

	eprint("execute_commands");
	if (!token)
		return (update_exit_status(shell, -1));
	token_count = arr_len_size(shell->token, sizeof(t_token));
	if (token_count == 1)
	{
		eprint("single token");
		exec_single_command(shell, token);
		update_exit_status(shell, shell->exit_status);
	}
	else
	{
		pipes = create_pipes(token_count);
		if (!pipes)
			return ;
		execute_pipes(shell, pipes, token_count);
	}
}
