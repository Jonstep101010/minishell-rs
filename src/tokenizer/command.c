#include <stdint.h>
#include "tokens.h"
#include "utils.h"
#include "commands.h"

void	set_cmd_func(t_token *token)
{
	uint8_t	i;

	i = 0;
	while (g_cmds[i].name)
	{
		token->builtin_info = i;
		if (occurs_exclusively(
				token->cmd_args[0].elem, g_cmds[i].name))
		{
			token->cmd_func = g_cmds[i].cmd;
			return ;
		}
		i++;
	}
	token->cmd_func = not_builtin;
	token->builtin_info = NOT_BUILTIN;
}
