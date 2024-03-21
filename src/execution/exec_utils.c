#include "utils.h"
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include "commands.h"

// handle execve fail:
void	execve_fail(t_shell *shell, char *cmd)
{
	eprint("%s: %s", cmd, strerror(errno));
	if (shell->env)
		arr_free(shell->env);
	destroy_all_tokens(shell);
	free(shell);
	exit(errno);
}
