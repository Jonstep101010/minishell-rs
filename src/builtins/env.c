#include "utils.h"
#include "libft.h"
#include <stddef.h>

int	builtin_env(char **envp)
{
	int	i = 0;
	if (!envp || !*(envp))
	{
		// printf("fatal error\n");
		return (1);
	}
	while (envp[i + 1])
	{
		printf("%s\n", envp[i]);
		i++;
	}
	return ((void)printf("%s\n", envp[i]), 0);
}
