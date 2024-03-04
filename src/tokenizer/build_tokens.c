#include <stddef.h>
#include <sys/param.h>
#include "arr_utils.h"
#include "commands.h"
#include "libft.h"
#include "tokens.h"
#include "utils.h"
#include "libutils.h"
#include "struct.h"
#include "parser.h"
#include "expander.h"

static void	*setup_token(t_token *token)
{
	size_t	len;

	if (!token || !token->split_pipes)
		return (NULL);
	token->tmp_arr = split_outside_quotes(token->split_pipes, WHITESPACE);
	if (!token->tmp_arr)
		return (NULL);
	len = arr_len((const char **)token->tmp_arr);
	if (len == 0)
		return (NULL);
	token->cmd_args = init_cmdargs(len);
	if (!token->cmd_args)
		return (NULL);
	return (token);
}

static void	set_arg_attributes(t_arg *cmd_arg)
{
	int		quote;
	char	*tmp;

	quote = 0;
	if (str_cchr(cmd_arg->elem, '\'') == 0
		&& str_cchr(cmd_arg->elem, '"') == 0)
		cmd_arg->quote = NONE;
	else if (str_cchr(cmd_arg->elem, '"'))
		cmd_arg->quote = DOUBLE;
	else if (str_cchr(cmd_arg->elem, '\''))
		cmd_arg->quote = SINGLE;
	if (cmd_arg->quote != NONE)
	{
		tmp = do_quote_bs(cmd_arg->elem, &quote);
		if (!tmp)
			return ;
		free(cmd_arg->elem);
		cmd_arg->elem = tmp;
	}
	cmd_arg->type = STRING;
}

static void	*expand_if_allowed(t_token *token, size_t ii, const char **envp)
{
	char	*tmp;

	if (token->cmd_func != builtin_env
		&& token->cmd_func != export
		&& token->cmd_func != unset)
	{
		tmp = expander(token->cmd_args[ii].elem, envp);
		if (!tmp)
			return (NULL);
		if (ft_strncmp(tmp, token->cmd_args[ii].elem, MAX(ft_strlen(tmp),
					ft_strlen(token->cmd_args[ii].elem)) == 0))
			free(tmp);
		else
		{
			free(token->cmd_args[ii].elem);
			token->cmd_args[ii].elem = tmp;
			tmp = expander(token->cmd_args[ii].elem, envp);
		}
		free(token->cmd_args[ii].elem);
		token->cmd_args[ii].elem = tmp;
	}
	return (token);
}

static void	*inner_loop(t_token *token, const char **envp)
{
	size_t	ii;

	ii = 0;
	while (token->tmp_arr[ii])
	{
		token->cmd_args[ii].elem = token->tmp_arr[ii];
		set_cmd_func(token);
		if (!expand_if_allowed(token, ii, envp))
			return (NULL);
		set_arg_attributes(&token->cmd_args[ii]);
		ii++;
	}
	return (token);
}

void	convert_split_token_string_array_to_tokens(t_shell *shell)
{
	size_t	i;

	i = 0;
	if (!shell->token || !shell->token[i].split_pipes)
		return ;
	while (shell->token[i].split_pipes)
	{
		setup_token(&shell->token[i]);
		inner_loop(&shell->token[i], (const char **)shell->owned_envp);
		free(shell->token[i].tmp_arr);
		i++;
	}
}
