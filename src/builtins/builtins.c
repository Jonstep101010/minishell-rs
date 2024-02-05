#include "ft_printf.h"
#include "struct.h"
#include <stdbool.h>
#include <sys/param.h>

char	*occurs_exclusively(const char *, const char *);

int		export(t_shell *shell, t_token *token);
char	**unset(t_shell *shell, t_token *token);
size_t	echo(char *cmd, char **args, char **envp);
int		builtin_env(char **envp);
// @follow-up parser needs to run before builtins in future,
// pass in only command char **
// need to add
int		builtin(t_shell *shell, t_token *token)
{
	char	buf[MAXPATHLEN + 1];
	if (!shell->owned_envp || !*(shell->owned_envp))
		return (-1);// something about env not existing
	if (!token->command || !*(token->command))
		return (-1);
	if (occurs_exclusively("echo", *token->command))
		return (echo(*token->command, token->command, shell->owned_envp));
	if (occurs_exclusively("unset", *token->command)
		&& token->command[1] && shell->owned_envp)
			if (!unset(shell, token))
				return (-1);
	if (occurs_exclusively("export", token->command[0]))
		return (export(shell, token));
	if (occurs_exclusively("pwd", token->command[0]))
	{
		getcwd(buf, MAXPATHLEN);
		return (ft_printf("%s\n", buf));
	}
	if (occurs_exclusively("env", token->command[0]))
		return (builtin_env(shell->owned_envp));
	return (0);
}

