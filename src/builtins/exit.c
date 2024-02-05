#include "struct.h"
#include "libft.h"

void	builtin_exit(t_shell *shell)
{
	int	exit_code;

	exit_code = shell->exit_status;
	if (shell->line)
		free_null(shell->line);
	ft_printf("exiting now...\n");
	if (shell->owned_envp)
		arr_free(shell->owned_envp);
	free_null(shell);
	exit(exit_code);
}
