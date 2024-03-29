#include <stdio.h>
#include <readline/readline.h>
#include "tokens.h"
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include "libft.h"
#include "utils.h"
#include "environment.h"

int	g_ctrl_c = 0;

static void	heredoc_loop(char *delim, int fd, char **env)
{
	char	*expanded;
	char	*line;

	expanded = NULL;
	line = NULL;
	g_ctrl_c = 0;
	while (1 && !g_ctrl_c)
	{
		line = readline("> ");
		if (equal(delim, line) || !line || g_ctrl_c)
		{
			g_ctrl_c = 0;
			break ;
		}
		if (ft_strchr(line, '$'))
		{
			expanded = expander(line, env);
			if (expanded && !equal(expanded, line))
				ft_putendl_fd(expanded, fd);
			free(expanded);
		}
		else
			ft_putendl_fd(line, fd);
		free(line);
	}
	free(line);
}

void	do_heredocs(t_token *token, int *target, char **env)
{
	int		fd;
	int		i;

	i = -1;
	while (token->cmd_args[++i].elem)
	{
		if (token->cmd_args[i].redir == HEREDOC)
		{
			fd = open(".heredoc.txt", O_RDWR | O_CREAT | O_TRUNC, 0644);
			if (fd == -1)
				return (eprint("%s", strerror(errno)), exit(1));
			heredoc_loop(token->cmd_args[i].elem, fd, env);
			close(fd);
			fd = open(".heredoc.txt", O_RDONLY);
			dup2(fd, *target);
			close(fd);
			unlink(".heredoc.txt");
		}
	}
}
