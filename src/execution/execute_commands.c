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
 * @brief is called by single command/pipe to execute the command
 * @audit should be renamed
 * @param shell
 * @param token pointer to element in array of tokens
 */
int	not_builtin(t_shell *shell, t_token *token)
{
	// @todo implement signals
	int			access_status;
	const char	**command = (const char **)get_cmd_arr_token(token);

	if (!command)
		exit_free(shell, 0);
	access_status = set_binpath(shell->env, token->cmd_args[0].elem, &(token->bin));
	if (access_status == 1 || access_status == 126 || access_status == 127)
		arr_free((char **)command);
	if (access_status == 1)
	{
		eprint("alloc fail");
		exit_free(shell, 1);
	}
	if (access_status == 126)
	{
		// eprint("%s: %s", token->cmd_args[0].elem, strerror(errno));
		exit_free(shell, 126);
	}
	if (access_status == 127)
	{
		eprint("command not found: %s", token->cmd_args[0].elem);
		exit_free(shell, 127);
	}
	if (execve(token->bin, (char **)command, shell->env) == -1)
	{
		arr_free((char **)command);
		execve_fail(shell, token->bin);
	}
	exit_free(shell, 0);
	return (0);
}

size_t	arr_len_size(void *arr, size_t size);

static bool forkable_builtin(t_token *token)
{
	return (token->cmd_func != builtin_exit
		&& token->cmd_func != builtin_export && token->cmd_func != builtin_unset
		&& token->cmd_func != builtin_cd);
}

void	execute_commands(t_shell *shell, t_token *token)
{
	int		token_count;
	int		redir_status;
	char	*error_elem;

	if (!token)
		return (update_exit_status(shell, -1));
	token_count = arr_len_size(shell->token, sizeof(t_token));
	if (token_count == 1 && !forkable_builtin(token))
	{
		redir_status = do_redirections(token->cmd_args, &error_elem);
		if (redir_status != 0)
		{
			eprint("%s: %s", error_elem, strerror(errno));
			return (update_exit_status(shell, redir_status));
		}
		update_exit_status(shell, token->cmd_func(shell, token));
	}
	else
		execute_pipes(shell, token_count);
	destroy_all_tokens(shell);
}
