#include "builtins.h"
#include "ft_printf.h"
#include "struct.h"
#include <stdbool.h>
#include <sys/param.h>

char	*occurs_exclusively(const char *, const char *);

int		export(t_shell *shell);

#include "env.h"

// @follow-up parser needs to run before builtins in future,
// pass in only command char **
// need to add
int		builtin(t_shell *shell)
{
	char	buf[MAXPATHLEN + 1];
	// if (!shell->owned_envp || !*(shell->owned_envp))
	// 	return (-1);
	if (!shell->command || !*(shell->command))
		return (-1);
	// print_arr(shell->owned_envp);
	if (occurs_exclusively("echo", *shell->command))
		return (echo(shell));
	if (occurs_exclusively("unset", *shell->command)
		&& shell->command[1] && shell->owned_envp)
	{
		shell->owned_envp = unset(shell->owned_envp, *(shell->command + 1));
		if (!shell->owned_envp)
			return (-1);
	}
	if (occurs_exclusively("export", *(shell->command)))
		return (export(shell));
	if (occurs_exclusively("pwd", *(shell->command)))
	{
		getcwd(buf, MAXPATHLEN);
		return (ft_printf("%s\n", buf));
	}
	if (occurs_exclusively("env", *(shell->command)))
	{
		if (!shell->owned_envp || !*(shell->owned_envp))
		{
			ft_printf("fatal error\n");
			exit(1);
		}
		int	i = 0;
		while (shell->owned_envp[i + 1])
		{
			ft_printf("%s\n", shell->owned_envp[i]);
			i++;
		}
		ft_printf("%s\n", shell->owned_envp[i]);
	}
	return (0);
}
