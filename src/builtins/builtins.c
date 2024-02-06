#include "ft_printf.h"
#include "struct.h"
#include <stdbool.h>
#include <sys/param.h>

char	*occurs_exclusively(const char *, const char *);

int		export(t_shell *shell, t_token *token);
char	**unset(t_shell *shell, const char *key);
size_t	echo(char *cmd, char **args, char **envp);
int		builtin_env(char **envp);
char	**split_outside_quotes(const char *to_split, char c);
void	add_pipe_split_as_tokens(char **pipe_split, t_shell *shell);
void	convert_split_token_string_array_to_tokens(t_shell *shell);
void	convert_tokens_to_string_array(t_token *token);
void	destroy_all_tokens(t_shell *shell);
#include <sys/wait.h>

// atm only works for execution of one command
void	execute_commands(t_shell *shell, t_token *token)
{
	pid_t test = -1;
	if (!occurs_exclusively("exit", token->cmd_args[0].elem))
		test = fork();
	if (test == 0)
	{
		if (execvp(token->cmd_args[0].elem, token->command) == -1)
			ft_printf("command not found\n");
		destroy_all_tokens(shell);
		exit(0);
	}
	else
		waitpid(test, NULL, 0);
}

void	builtin_exit(t_shell *shell);
// @follow-up parser needs to run before builtins in future,
// pass in only command char **
// need to add
int		builtin(t_shell *shell, t_token *token)
{
	if (!token)
		return (-1);
	convert_tokens_to_string_array(shell->token);
	char	buf[MAXPATHLEN + 1];

	printf("my builtins:\n");
	if (!token || !token->command || !*token->command)
		return (-1);
	// if (!shell->owned_envp || !*(shell->owned_envp))
	// 	return (-1);// something about env not existing
	// if (!token->command || !*(token->command))
	// 	return (-1);
	if (occurs_exclusively("echo", *token->command))
		return (echo(*token->command, token->command, shell->owned_envp));
	if (occurs_exclusively("unset", *token->command)
		&& token->command[1] && shell->owned_envp)
	{
		if (arr_len((const char **)token->command) > 2)
			return(ft_printf("Error: too many arguments!\n"), -1);
		if (!unset(shell, token->command[1]))
			return (-1);
		return (0);
	}
	if (occurs_exclusively("export", token->command[0]))
		return (export(shell, token));
	if (occurs_exclusively("pwd", token->command[0]))
	{
		getcwd(buf, MAXPATHLEN);
		return (ft_printf("%s\n", buf));
	}
	if (occurs_exclusively("env", token->command[0]))
		return (builtin_env(shell->owned_envp));
	printf("builtin not found: running exec!\n");
	execute_commands(shell, token);
	return (0);
}

