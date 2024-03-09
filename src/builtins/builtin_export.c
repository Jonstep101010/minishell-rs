#include "struct.h"
#include "environment.h"
#include <stdio.h>
#include <unistd.h>
#include "tokens.h"
#include "str_utils.h"
#include "utils.h"
#include "commands.h"
#include "libft.h"

static int	declare_x_env_var(char *const *env)
{
	while (*env)
	{
		printf("declare -x %s\n", *env);
		env++;
	}
	return (0);
}

int	builtin_export(t_shell *shell, t_token *token)
{
	size_t	i;

	i = 1;
	if (!token->command || !token->command[i])
		return (declare_x_env_var(shell->env));
	while (token->command[i])
	{
		if (!check_valid_key(token->command[i])
			|| str_cchr(token->command[i], '=') != 1)
		{
			eprint("export: '%s': not a valid identifier\n", token->command[i]);
			return (1);
		}
		export_env(shell, ft_strdup(token->command[i]));
		i++;
	}
	if (i > 1)
		return (0);
	return (1);
}
