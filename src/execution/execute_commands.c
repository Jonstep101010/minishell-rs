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

uint8_t	set_binpath(char *const *env, const char *bin, char **binpath_buf);

/**
 * @brief
 * @audit atm only works for execution of one command (forbidden function)
 * @audit should be renamed
 * @param shell
 * @param token pointer to element in array of tokens
 */
int	not_builtin(t_shell *shell, t_token *token)
{
	// @todo implement signals
	int	access_status;
	access_status = set_binpath(shell->env, token->cmd_args[0].elem, &(token->bin));
	eprint("access_status %d", access_status);
	if (access_status == 1)
	{
		eprint("alloc fail");
		exit_free(shell, 1);
	}
	if (access_status == 126)
	{
		eprint("%s: %s", token->cmd_args[0].elem, strerror(errno));
		exit_free(shell, 126);
	}
	if (access_status == 127)
	{
		eprint("command not found: %s", token->cmd_args[0].elem);
		exit_free(shell, 127);
	}
	if (execve(token->bin, token->command, shell->env) == -1)
		execve_fail(shell, token->bin);
	exit_free(shell, 0);
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

static bool forkable_builtin(t_token *token)
{
	return (token->cmd_func != builtin_exit
		&& token->cmd_func != builtin_export && token->cmd_func != builtin_unset
		&& token->cmd_func != builtin_cd);
}

static void	exec_single_command(t_shell *shell, t_token *token)
{
	pid_t	pid;
	int		status;
	int		redir_status;

	if (!forkable_builtin(token))
	{
		// eprint("not forkable builtin");
		redir_status = do_redirections(token->cmd_args);
		if (redir_status != 0)
			return (update_exit_status(shell, redir_status));
		convert_tokens_to_string_array(token);
		update_exit_status(shell, token->cmd_func(shell, token));
		destroy_all_tokens(shell);
		return ;
	}
	// eprint("forkable function");
	pid = fork();
	if (pid < 0)
		return (eprint("fork %s", strerror(errno)));
	eprint("first elem %s", token->cmd_args[0].elem);
	if (pid == 0)
	{
		if (token->has_redir)
			heredoc_nopipe(token, shell->env);
		do_redirections(token->cmd_args);
		convert_tokens_to_string_array(token);
		exit_free(shell, token->cmd_func(shell, token));
	}
	else
	{
		waitpid(pid, &status, 0);
		if (WIFEXITED(status))
			update_exit_status(shell, WEXITSTATUS(status));
		destroy_all_tokens(shell);
	}
}

void	execute_commands(t_shell *shell, t_token *token)
{
	int	token_count;
	int	**pipes;

	// eprint("execute_commands");
	if (!token)
		return (update_exit_status(shell, -1));
	token_count = arr_len_size(shell->token, sizeof(t_token));
	if (token_count == 1)
	{
		// eprint("single token");
		exec_single_command(shell, token);
	}
	else
	{
		pipes = create_pipes(token_count);
		if (!pipes)
			return ;
		execute_pipes(shell, pipes, token_count);
	}
}
