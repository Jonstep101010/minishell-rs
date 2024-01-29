#ifndef PARSER_H
# define PARSER_H

# include "struct.h"
int		parser(t_shell *shell);

void	*split_command(t_shell *shell);

void	expand_variables(t_shell *shell);

bool	interpret_quotes(char **cmd_arr);

#endif