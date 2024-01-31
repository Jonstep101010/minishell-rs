#ifndef BUILTINS_H
# define BUILTINS_H

#include "struct.h"

int		echo(t_shell *shell);
int		builtin(t_shell *shell);
char	*occurs(const char *big, const char *little);

#endif