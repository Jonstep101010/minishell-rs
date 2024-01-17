#include "struct.h"
#include "libft.h"

void	msh_exit(t_shell *shell, int exitcode)
{
	if (shell->line)
		free_null(shell->line);
	ft_printf("exiting now...\n");
	if (shell->owned_envp)
		arr_free(shell->owned_envp);
	free_null(shell);
	exit(exitcode);
}
