#ifndef PARSER_H
# define PARSER_H

# include "struct.h"
int		parser(t_shell *shell);

void	*split_command(t_shell *shell);

char	*expand_variables(char *line, char **envp);

bool	interpret_quotes(char **cmd_arr);

#endif