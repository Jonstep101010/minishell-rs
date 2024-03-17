#include "utils.h"
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include "commands.h"

// handle execve fail:
void	execve_fail(t_shell *shell, char *cmd)
{
	eprint("%s: %s\n", cmd, strerror(errno));
	shell->exit_status = errno;
	builtin_exit(shell, NULL);
}
