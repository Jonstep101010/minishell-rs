#include "struct.h"
#include "environment.h"
#include <stdio.h>
#include <unistd.h>
#include "tokens.h"
#include "libutils.h"
#include "utils.h"
#include "commands.h"

static int	declare_x_env_var(char *const *env)
{
	while (*env)
	{
		printf("declare -x %s\n", *env);
		env++;
	}
	return (0);
}

int	export(t_shell *shell, t_token *token)
{
	size_t	i;

	i = 1;
	if (!token->command[i])
		return (declare_x_env_var(shell->owned_envp));
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
