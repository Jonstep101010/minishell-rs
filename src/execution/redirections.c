#include "tokens.h"
#include "utils.h"
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <unistd.h>

static int	open_redir(const char *file, enum e_redir redir)
{
	int		fd;
	bool	perm;

	fd = -2;
	perm = false;
	if (redir == INPUT_REDIR)
	{
		perm = access(file, R_OK | F_OK);
		if (perm)
			fd = open(file, O_RDONLY);
	}
	else if (redir == OUTPUT_REDIR)
	{
		perm = access(file, W_OK);
		if (perm)
			fd = open(file, O_WRONLY | O_CREAT | O_TRUNC, 0644);
	}
	else if (redir == APPEND)
	{
		perm = access(file, W_OK);
		if (perm)
			fd = open(file, O_WRONLY | O_CREAT | O_APPEND, 0644);
	}
	return (fd);
}

/**
 * @brief
 *
 * @param token
 * @return int 0 if no error, 1 if open failed, 2 if access failed
 */
int	do_redirections(t_arg *cmd_args)
{
	int		fd;
	int		i;

	i = 0;
	while (cmd_args[i].elem)
	{
		if (cmd_args[i].redir == INPUT_REDIR
			|| cmd_args[i].redir == OUTPUT_REDIR
				|| cmd_args[i].redir == APPEND)
		{
			fd = open_redir(cmd_args[i].elem, cmd_args[i].redir);
			if (fd < 0)
			{
				eprint("%s: %s", cmd_args[i].elem, strerror(errno));
				if (fd == -2)
					return (2);
				return (1);
			}
			if (cmd_args[i].redir != INPUT_REDIR)
				dup2(fd, STDOUT_FILENO);
			else
				dup2(fd, STDIN_FILENO);
			close(fd);
		}
		i++;
	}
	return (0);
}
