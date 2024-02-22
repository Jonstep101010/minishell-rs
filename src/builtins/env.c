#include "struct.h"
#include "tokens.h"
#include <stdio.h>

int	export_env(const char **envp)
{
	while (*envp)
	{
		printf("declare -x %s\n", *envp);
		envp++;
	}
	return (0);
}

int	builtin_env(t_shell *shell, t_token *token)
{
	const char	**envp = (const char **)shell->owned_envp;

	(void)token;
	while (*envp)
	{
		printf("%s\n", *envp);
		envp++;
	}
	return (0);
}
