#ifndef PARSER_H
# define PARSER_H

# include "struct.h"
int		parser(t_shell *shell);

char	**split_command(t_shell *shell);

typedef struct s_expander
{
	size_t	i;
	char	*ret;
	char	*key;
	size_t	start;
	char	*val;
	char	*tmp;
	char	*remainder_line;
	char	*new_ret;
	int		singlequote;
}	t_expander;

char	*expand_variables(char *line, const char **envp);

bool	interpret_quotes(char **cmd_arr);

#endif