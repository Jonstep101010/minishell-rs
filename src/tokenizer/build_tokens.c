#include <assert.h>
#include <stddef.h>
#include <sys/param.h>
#include "arr_utils.h"
#include "commands.h"
#include "libft.h"
#include "tokens.h"
#include "utils.h"
#include "libutils.h"
#include "environment.h"
#include "struct.h"
#include "parser.h"

void	rm_prefix_redir_word(t_arg *arg)
{
	size_t i;
	size_t len;

	if (!arg)
		return;

	i = 0;
	len = 0;
	while (arg[len].elem)
		len++;
	while (arg[i].elem)
	{
		if (arg[i].type == REDIR_REMOVED)
		{
			free(arg[i].elem);
			// @follow-up fix?
			// arg[i + 1].type = REDIR;
			// arg[i + 1].redir = arg[i].redir;
			while (i < len)
			{
				arg[i] = arg[i + 1];
				i++;
			}
			return (rm_prefix_redir_word(arg));
		}
		i++;
	}
}

// currently needs to be called after check_redirections and before rm_prefix_redir_word
void	parse_redir_types(t_arg *arg)
{
	int		i;
	char	*tmp;

	i = -1;
	while (arg[++i].elem)
	{
		if (arg[i].type == REDIR_WORD)
			continue ;
		if (arg[i].type == REDIR)
		{
			if (arg[i].redir == INPUT_REDIR
				|| arg[i].redir == OUTPUT_REDIR)
				tmp = ft_strdup(&arg[i].elem[1]);
			else
				tmp = ft_strdup(&arg[i].elem[2]);
			free(arg[i].elem);
			arg[i].elem = tmp;
		}
		if (arg[i].type == REDIR_REMOVED)
		{
			arg[i + 1].type = REDIR_WORD;
			arg[i + 1].redir = arg[i].redir;
		}
	}
}

static void	set_type_redir(t_arg *cmd_arg)
{
	if (cmd_arg->redir == APPEND
			|| cmd_arg->redir == HEREDOC)
	{
		if (!cmd_arg->elem[2])
			cmd_arg->type = REDIR_REMOVED;
		else
			cmd_arg->type = REDIR;
	}
	else if (cmd_arg->redir == OUTPUT_REDIR
			|| cmd_arg->redir == INPUT_REDIR)
	{
		if (!cmd_arg->elem[1])
			cmd_arg->type = REDIR_REMOVED;
		else
			cmd_arg->type = REDIR;
	}
}
// cannot be inside quotes at this point (we compare max 2 chars)
enum e_redir	check_redirections(t_arg *cmd_args)
{
	size_t	ii;
	bool	redir;

	ii = 0;
	redir = false;
	while (cmd_args[ii].elem)
	{
		cmd_args[ii].redir = NO_REDIR;
		cmd_args[ii].type = STRING;
		if (ft_strncmp(cmd_args[ii].elem, ">>", 2) == 0)
			cmd_args[ii].redir = APPEND;
		else if (ft_strncmp(cmd_args[ii].elem, ">", 1) == 0)
			cmd_args[ii].redir = OUTPUT_REDIR;
		else if (ft_strncmp(cmd_args[ii].elem, "<<", 2) == 0)
			cmd_args[ii].redir = HEREDOC;
		else if (ft_strncmp(cmd_args[ii].elem, "<", 1) == 0)
			cmd_args[ii].redir = INPUT_REDIR;
		if (cmd_args[ii].redir != NO_REDIR)
		{
			set_type_redir(&cmd_args[ii]);
			redir = true;
		}
		ii++;
	}
	return (redir);
}

static void	*setup_token(t_token *token)
{
	size_t	len;

	if (!token || !token->split_pipes)
		return (NULL);
	token->tmp_arr = split_outside_quotes(token->split_pipes, WHITESPACE);
	if (!token->tmp_arr)
		return (NULL);
	len = arr_len(token->tmp_arr);
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
}

static void	*expand_if_allowed(t_token *token, size_t ii, char *const *env)
{
	char	*tmp;

	if (token->cmd_func != builtin_env
				&& str_cchr(token->cmd_args[ii].elem, '$') != 0)
	{
		tmp = expander(token->cmd_args[ii].elem, env);
		if (!tmp)
			return (NULL);
		if (ft_strncmp(tmp, token->cmd_args[ii].elem, MAX(ft_strlen(tmp),
					ft_strlen(token->cmd_args[ii].elem)) == 0))
			free(tmp);
		else
		{
			free(token->cmd_args[ii].elem);
			token->cmd_args[ii].elem = tmp;
		}
	}
	return (token);
}

static void	*inner_loop(t_token *token, char *const *env)
{
	size_t	ii;

	ii = 0;
	while (token->tmp_arr[ii])
	{
		token->cmd_args[ii].elem = token->tmp_arr[ii];
		set_cmd_func(token);
		if (!expand_if_allowed(token, ii, env))
			return (NULL);
		set_cmd_func(token);
		set_arg_attributes(&token->cmd_args[ii]);// last step
		ii++;
	}
	if (check_redirections(&token->cmd_args[0]))
	{
		size_t	tmp = 0;
		parse_redir_types(&token->cmd_args[0]);
		rm_prefix_redir_word(&token->cmd_args[0]);
		while (token->cmd_args[tmp].elem)
		{
			if (token->cmd_args[tmp].type == REDIR_WORD)
				token->cmd_args[tmp].type = REDIR;
			tmp++;
		}
	}
	return (token);
}

void	*tokenize(t_shell *shell, char const *trimmed_line)
{
	size_t	i;

	i = 0;
	shell->token = get_tokens(trimmed_line);
	if (!shell->token || !shell->token->split_pipes
			|| !shell->token->split_pipes[0])
		return (eprint("alloc fail"), NULL);
	while (shell->token[i].split_pipes)
	{
		setup_token(&shell->token[i]);
		inner_loop(&shell->token[i], shell->env);
		free(shell->token[i].tmp_arr);
		i++;
	}
	return (shell->token);
}
