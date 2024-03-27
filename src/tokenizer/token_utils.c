#include <stdint.h>
#include "tokens.h"
#include "utils.h"
#include "commands.h"
#include "libft.h"

void	set_cmd_func(const char *cmd, t_token *token)
{
	uint8_t					i;
	const struct s_func		cmds[] = {
	{"echo", echo}, {"cd", builtin_cd},
	{"pwd", builtin_pwd}, {"export", builtin_export},
	{"unset", builtin_unset}, {"env", builtin_env},
	{"exit", builtin_exit}, {NULL, NULL}};

	i = 0;
	while (cmds[i].name)
	{
		if (equal(cmd, cmds[i].name))
		{
			token->cmd_func = cmds[i].cmd;
			return ;
		}
		i++;
	}
	token->cmd_func = exec_bin;
}

t_arg	*init_cmdargs(size_t size)
{
	const t_arg	*args = ft_calloc(sizeof(t_arg), size + 1);
	const t_arg	template = {
		.redir = NO_REDIR,
		.type = STRING
	};

	while (args && size--)
		ft_memcpy((t_arg *)&args[size], &template, sizeof(*args));
	return ((t_arg *)args);
}

/**
 * @param size must be size of non-NULL, terminated array
 * @return t_token* instance
 */
t_token	*init_token(size_t size)
{
	const t_token	*token = ft_calloc(sizeof(t_token), (size + 1));
	const t_token	template = {
		.has_redir = false,
		.cmd_func = exec_bin,
	};

	while (token && size--)
		ft_memcpy((t_token *)&token[size], &template, sizeof(*token));
	return ((t_token *)token);
}
