#include "utils.h"
#include "libft.h"
#include <stddef.h>

int	builtin_env(char **envp)
{
	int	i = 0;
	if (!envp || !*(envp))
	{
		ft_printf("fatal error\n");
		return (1);
	}
	while (envp[i + 1])
	{
		ft_printf("%s\n", envp[i]);
		i++;
	}
	return ((void)ft_printf("%s\n", envp[i]), 0);
}
