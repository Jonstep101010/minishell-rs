#include <stdint.h>
#include "tokens.h"
#include "utils.h"
#include "commands.h"
#include "libft.h"

void	set_cmd_func(t_token *token)
{
	uint8_t	i;

	i = 0;
	while (g_cmds[i].name)
	{
		if (equal(
				token->cmd_args[0].elem, g_cmds[i].name))
		{
			token->cmd_func = g_cmds[i].cmd;
			return ;
		}
		i++;
	}
	token->cmd_func = not_builtin;
}

t_arg	*init_cmdargs(size_t size)
{
	t_arg	*args;

	if (size == 0)
		return (NULL);
	args = ft_calloc(sizeof(t_arg), (size + 1));
	if (!args)
		return (NULL);
	return (args);
}

t_token	*init_token(size_t size)
{
	t_token	*token;

	token = ft_calloc(sizeof(t_token), (size + 1));
	if (!token)
		return (NULL);
	token[size].cmd_args = NULL;
	token[size].tmp_arr = NULL;
	token[size].command = NULL;
	token[size].split_pipes = NULL;
	token[size].cmd_func = NULL;
	return (token);
}
