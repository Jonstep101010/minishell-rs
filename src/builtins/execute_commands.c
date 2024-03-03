#include "struct.h"
#include "tokens.h"
#include "utils.h"
#include <stdbool.h>
#include <sys/param.h>
#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>

/**
 * @brief
 * @audit atm only works for execution of one command (forbidden function)
 * @param shell
 * @param token pointer to element in array of tokens
 */
int	not_builtin(t_shell *shell, t_token *token)
{
	pid_t test = -1;
	if (!equal("exit", token->cmd_args[0].elem))
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
	return (0);
}

// @follow-up could be called from a loop and destroy owned args?
void	execute_commands(t_shell *shell, t_token *token)
{
	convert_tokens_to_string_array(token);
	if (!token || !token->command || !*token->command)
		update_exit_status(shell, -1);
	update_exit_status(shell, token->cmd_func(shell, token));
}
