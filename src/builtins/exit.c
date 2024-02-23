#include "struct.h"
#include "arr_utils.h"
#include <stdlib.h>

void	destroy_all_tokens(t_shell *shell);

void	builtin_exit(t_shell *shell, t_token *null)
{
	int		exit_code;

	(void)null;
	exit_code = 0;
	if (shell->owned_envp)
		arr_free(shell->owned_envp);
	free(shell);
	exit(exit_code);
}
