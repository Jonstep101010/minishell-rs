#include <stdio.h>

int	export_env(const char **envp)
{
	int	i;

	i = 0;
	while (envp[i])
	{
		printf("declare -x %s\n", envp[i]);
		i++;
	}
	return (0);
}

int	builtin_env(char **envp)
{
	int	i = 0;
	if (!envp || !*(envp))
	{
		// printf("fatal error\n");
		return (1);
	}
	while (envp[i])
	{
		printf("%s\n", envp[i]);
		i++;
	}
	return (0);
}
