#ifndef BUILTINS_H
# define BUILTINS_H

#include "struct.h"

int		echo(const char **cmd_arr);
int		builtin(t_shell *shell, const char **cmd_arr);
char	*occurs(const char *big, const char *little);

#endif