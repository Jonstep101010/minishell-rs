#include "struct.h"
#include "arr_utils.h"
#include <stdlib.h>
#include "utils.h"
#include "libft.h"

int		builtin_exit(t_shell *shell, t_token *code_nullable)
{
	int		exit_code;

	exit_code = shell->exit_status;
	if (code_nullable && code_nullable->command)
	{
		if (code_nullable->command[1] && code_nullable->command[2])
		{
			eprint("exit: too many arguments");
			shell->exit_status = 1;
			return (1);
		}
		if (code_nullable->command[1])
		{
			exit_code = ft_atoi(code_nullable->command[1]);
			if (exit_code < 0)
				exit_code += 256;
		}
		eprint_single("exit\n", 2);
	}
	if (shell->owned_envp)
		arr_free(shell->owned_envp);
	destroy_all_tokens(shell);
	free(shell);
	exit(exit_code);
}
