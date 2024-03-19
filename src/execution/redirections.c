#include "tokens.h"
#include "utils.h"
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>

void	do_redirections(t_token *token)
{
	int		fd;
	int		i;

	i = 0;
	while (token->cmd_args[i].elem)
	{
		if (token->cmd_args[i].type == REDIR && token->cmd_args[i].redir != HEREDOC)
		{
			eprint("redir elem: %s\n", token->cmd_args[i].elem);
			fd = -2;
			if (token->cmd_args[i].redir == INPUT_REDIR)
				fd = open(token->cmd_args[i].elem, O_RDONLY);
			else if (token->cmd_args[i].redir == OUTPUT_REDIR)
			{
				fd = open(token->cmd_args[i].elem, O_WRONLY | O_CREAT | O_TRUNC, 0644);
			}
			else if (token->cmd_args[i].redir == APPEND)
				fd = open(token->cmd_args[i].elem, O_WRONLY | O_CREAT | O_APPEND, 0644);
			if (fd == -1)
				eprint("open %s\n", strerror(errno));
			else if (token->cmd_args[i].redir != INPUT_REDIR)
				dup2(fd, STDOUT_FILENO);
			else
				dup2(fd, STDIN_FILENO);
			close(fd);
		}
		i++;
	}
}
