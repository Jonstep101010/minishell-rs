#include "struct.h"
#include "environment.h"
#include <stdio.h>
#include <unistd.h>
#include "tokens.h"
#include "libutils.h"
#include "utils.h"

int	export_env(const char **envp);
#include "commands.h"

int	export(t_shell *shell, t_token *token)
{
	size_t	i;

	i = 1;
	if (!token->command[i])
		return (export_env((const char **)shell->owned_envp));
	while (token->command[i])
	{
		if (!check_valid_key(token->command[i])
			|| str_cchr(token->command[i], '=') != 1)
		{
			eprint("export: '%s': not a valid identifier\n", token->command[i]);
			return (1);
		}
		shell->owned_envp = export_var(shell->owned_envp, token->command[i]);
		if (!shell->owned_envp)
		{
			eprint("fatal: enviroment invalidated\n");
			shell->exit_status = 1;
			builtin_exit(shell, NULL);
		}
		i++;
	}
	if (i > 1)
		return (0);
	return (1);
}
