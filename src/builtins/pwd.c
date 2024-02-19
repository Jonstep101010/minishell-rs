#include "environment.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int	builtin_pwd(const char **envp)
{
	char	*tmp;
	char	*env_pwd;

	tmp = getcwd(NULL, 0);
	env_pwd = get_env_var(envp, "PWD");
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
