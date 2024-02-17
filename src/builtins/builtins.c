#include "struct.h"
#include <stdbool.h>
#include <sys/param.h>
#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include "builtins.h"

/**
 * @brief
 * @audit atm only works for execution of one command (forbidden function)
 * @param shell
 * @param token pointer to element in array of tokens
 */
void	execute_commands(t_shell *shell, t_token *token)
{
	pid_t test = -1;
	if (!occurs_exclusively("exit", token->cmd_args[0].elem))
		test = fork();
	if (test == 0)
	{
		// if (execvp(token->cmd_args[0].elem, token->command) == -1)
		// 	printf("command not found\n");
		execvp(token->cmd_args[0].elem, token->command);
		destroy_all_tokens(shell);
		exit(0);
	}
	else
		waitpid(test, NULL, 0);
}

// @follow-up parser needs to run before builtins in future,
// pass in only command char **
// need to add
int		builtin(t_shell *shell, t_token *token)
{
	if (!token)
		return (-1);
	convert_tokens_to_string_array(shell->token);

	if (!token || !token->command || !*token->command)
		return (-1);
	// if (!shell->owned_envp || !*(shell->owned_envp))
	// 	return (-1);// something about env not existing
	// if (!token->command || !*(token->command))
	// 	return (-1);
	if (occurs_exclusively("echo", *token->command))
		return (echo(*token->command, token->command, shell->owned_envp));
	if (occurs_exclusively("unset", *token->command) && shell->owned_envp)
		return (unset((const char **)token->command, shell->owned_envp));
	if (occurs_exclusively("export", token->command[0]))
		return (export(shell, token));
	if (occurs_exclusively("pwd", token->command[0]))
		return (builtin_pwd((const char **)shell->owned_envp));
	if (occurs_exclusively("env", token->command[0]))
		return (builtin_env(shell->owned_envp));
	if (occurs_exclusively("cd", token->command[0]))
		return (builtin_cd(token->command, shell->owned_envp));
	printf("builtin not found: running exec!\n");
	execute_commands(shell, token);
	return (0);
}
