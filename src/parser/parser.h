#ifndef PARSER_H
# define PARSER_H

# include "struct.h"
int		parser(t_shell *shell);

typedef struct s_splitter
{
	int		quote;
	size_t	i;
	size_t	start;
	size_t	len;
	char	**ret;
	char	*tmp;
}	t_splitter;

char	*expand_variables(const char *line, const char **envp);

bool	interpret_quotes(char **cmd_arr);

#endif