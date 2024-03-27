#include "tokens.h"
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <unistd.h>

static int	open_redir(const char *file, enum e_redir redir, int *fd)
{
	int		perm;

	if (redir == INPUT_REDIR)
	{
		perm = access(file, F_OK);
		if (perm != 0)
			return (127);
		perm = access(file, R_OK);
		if (perm != 0)
			return (126);
		*fd = open(file, O_RDONLY);
	}
	else if (redir == OUTPUT_REDIR)
		*fd = open(file, O_WRONLY | O_CREAT | O_TRUNC, 0644);
	else if (redir == APPEND)
	{
		perm = access(file, W_OK);
		if (perm != 0)
			return (errno);
		*fd = open(file, O_WRONLY | O_CREAT | O_APPEND, 0644);
	}
	if (*fd == -1)
		return (errno);
	return (0);
}

/**
 * @brief
 *
 * @param token
 * @return int 0 if no error, 1 if open failed, 2 if access failed
 */
int	do_redirections(t_arg *cmd_args, char **error_elem)
{
	int	i;
	int	fd;

	i = 0;
	while (cmd_args[i].elem)
	{
		fd = 0;
		if (cmd_args[i].type == REDIR && cmd_args[i].redir != HEREDOC)
		{
			if (open_redir(cmd_args[i].elem, cmd_args[i].redir, &fd) != 0)
			{
				*error_elem = cmd_args[i].elem;
				return (errno);
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
