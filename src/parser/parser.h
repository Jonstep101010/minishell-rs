#ifndef PARSER_H
# define PARSER_H

# include "struct.h"
int		parser(t_shell *shell);

typedef struct s_expander
{
	size_t	i;
	char	*ret;
	char	*key;
	size_t	start;
	char	*val;
	char	*tmp;
	int		singlequote;
	char	*remainder_line;
	char	*line;
}	t_expander;

typedef struct s_splitter
{
	int		quote;
	size_t	i;
	size_t	start;
	size_t	len;
	char	**ret;
	char	*tmp;
	char	**arr;
}	t_splitter;

char	*expand_variables(const char *input, const char **envp);

bool	interpret_quotes(char **cmd_arr);

#endif