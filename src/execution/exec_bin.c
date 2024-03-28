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
 * @brief is called for each token (not builtin)
 */
int	exec_bin(t_shell *shell, t_token *token)
{
	// @todo implement signals
	int			access_status;
	const char	**command = (const char **)get_cmd_arr_token(token);

	if (!command)
		exit_free(shell, 0);
	access_status = set_binpath(shell->env, *command, &(token->bin));
	if (access_status == 1)
	{
		eprint("alloc fail");
		exit_free(shell, 1);
	}
	if (access_status == 126)
	{
		// @todo implement strerror, exit codes
		eprint("%s: %s", *command, strerror(errno));
		arr_free((char **)command);
		exit_free(shell, 126);
	}
	if (access_status == 127)
	{
		eprint("%s: command not found", *command);
		arr_free((char **)command);
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
