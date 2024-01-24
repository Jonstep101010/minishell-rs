#include "builtins.h"
#include "ft_printf.h"
#include "struct.h"
#include "libft.h"
#include <stdbool.h>
#include <sys/param.h>

char	*occurs_exclusively(const char *, const char *);
int		export(char **owned_envp, const char **cmd_arr);
// @follow-up parser needs to run before builtins in future,
// pass in only command char **
// need to add
int		builtin(t_shell *shell, const char **cmd_arr)
{
	char	buf[MAXPATHLEN + 1];
	if (!cmd_arr || !*(cmd_arr))
		return (-1);
	print_arr(cmd_arr);
	if (occurs_exclusively("echo", *cmd_arr))
		return (echo(cmd_arr));
	// @todo implement unset command
	// if (occurs_exclusively("unset", *cmd_arr))
	// 	return (unset(shell->owned_envp, cmd_arr));
	if (occurs_exclusively("export", *cmd_arr))
		return (export(shell->owned_envp, cmd_arr));
	if (occurs_exclusively("pwd", *cmd_arr))
	{
		getcwd(buf, MAXPATHLEN);
		return (ft_printf("%s\n", buf));
	}
	return (-1);
}
