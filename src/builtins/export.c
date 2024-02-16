#include "struct.h"
#include "libft.h"
#include "environment.h"
#include <stdio.h>
#include <unistd.h>
#include "tokens.h"

int	export(t_shell *shell, t_token *token)
{
	size_t	i;

	i = 1;
	if (!token->command[i])
	{
		// @todo handle export without arguments?
		return (0);
	}
	while (token->command[i])
	{
		if (!check_valid_key(token->command[i])
			|| str_cchr(token->command[i], '=') != 1)
			return (dprintf(STDERR_FILENO, "minishell: export: '%s': invalid variable name\n", token->command[i]), 1);
		shell->tmp_arr = export_var(shell->owned_envp, token->command[i]);
		if (!shell->tmp_arr)
			return (0);
		if (shell->owned_envp != shell->tmp_arr)
			arr_free(shell->owned_envp);
		shell->owned_envp = shell->tmp_arr;
		shell->tmp_arr = NULL;
		i++;
	}
	if (i > 1)
		return (0);
	return (1);
}
