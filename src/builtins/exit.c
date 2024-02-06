#include "struct.h"
#include "libft.h"

void	destroy_all_tokens(t_shell *shell);
void	builtin_exit(t_shell *shell)
{
	int	exit_code;

	exit_code = 0;
	if (shell->line)
		free_null(shell->line);
	ft_printf("exiting now...\n");
	if (shell->owned_envp)
		arr_free(shell->owned_envp);
	size_t	i = 0;
	while (shell->token && shell->token[i].split_pipes)
	{
		if (shell->token[i].command)
			arr_free(shell->token[i].command);
		i++;
	}
	destroy_all_tokens(shell);
	free(shell);
	exit(exit_code);
}
