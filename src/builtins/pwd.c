#include "environment.h"
#include "struct.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int	builtin_pwd(t_shell *shell, t_token *token_nullable)
{
	char	*tmp;
	char	*env_pwd;

	(void)token_nullable;
	tmp = getcwd(NULL, 0);
	env_pwd = get_env_var((const char **)shell->owned_envp, "PWD");
	if (!env_pwd)
		printf("%s\n", tmp);
	else
		printf("%s\n", env_pwd);
	if (tmp)
		free(tmp);
	if (env_pwd)
		free(env_pwd);
	return (0);
}
