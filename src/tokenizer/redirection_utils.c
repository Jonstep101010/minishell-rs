#include "libutils.h"
#include "tokens.h"
#include "libft.h"
#include <stdbool.h>

void	rm_prefix_redir_word(t_arg *arg)
{
	size_t i;
	size_t len;

	if (!arg)
		return ;
	i = 0;
	len = memsize(arg, sizeof(t_arg));
	while (arg[i].elem)
	{
		if (arg[i].type == REDIR_REMOVED && arg[i + 1].elem)
		{
			free(arg[i].elem);
			arg[i + 1].type = REDIR;
			arg[i + 1].redir = arg[i].redir;
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
		if (arg[i].type == REDIR)
		{
			if (arg[i].redir == INPUT_REDIR
				|| arg[i].redir == OUTPUT_REDIR)
				tmp = ft_strdup(&arg[i].elem[1]);
			else
				tmp = ft_strdup(&arg[i].elem[2]);
			free_null(&(arg[i].elem));
			arg[i].elem = tmp;
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
