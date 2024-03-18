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
	pid_t 	test;
	int		status;

	test = fork();
	if (test == -1)
		eprint("fork %s\n", strerror(errno));
	if (test == 0)
	{
		// implement signals, redirs
		if (execvp(token->cmd_args[0].elem, token->command) == -1)
			execve_fail(shell, token->cmd_args[0].elem);
	}
	else
	{
		waitpid(test, &status, 0);
		if (WIFEXITED(status))
			shell->exit_status = WEXITSTATUS(status);
	}
	return (0);
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

// @follow-up could be called from a loop and destroy owned args?
void	execute_commands(t_shell *shell, t_token *token)
{
	int	token_count;
	int	**pipes;

	if (!token)
		return ;
	token_count = arr_len_size(shell->token, sizeof(t_token));
	if (token_count == 1)
	{
		convert_tokens_to_string_array(token);
		if (!token || !token->command || !*token->command)
			update_exit_status(shell, -1);
		update_exit_status(shell, token->cmd_func(shell, token));
		return ;
	}
	pipes = create_pipes(token_count);
	if (!pipes)
		return ;
	execute_pipes(shell, pipes, token_count);
}
