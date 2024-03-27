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

size_t	memsize(void *arr, size_t size);

static bool forkable_builtin(t_token *token)
{
	return (token->cmd_func != builtin_exit
		&& token->cmd_func != builtin_export
		&& token->cmd_func != builtin_unset
		&& token->cmd_func != builtin_cd);
}

void	execute_commands(t_shell *shell, t_token *token)
{
	int		token_count;
	int		redir_status;
	char	*error_elem;

	if (!token)
		return (update_exit_status(shell, -1));
	token_count = memsize(shell->token, sizeof(t_token));
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
