#include "struct.h"
#include "tokens.h"
#include <stdio.h>

/**
 * @brief print all environment variables
 */
int	builtin_env(t_shell *shell, t_token *token)
{
	const char *const *env = (const char *const *)shell->env;

	(void)token;
	while (*env)
	{
		printf("%s\n", *env);
		env++;
	}
	return (0);
}
