#ifndef BUILTINS_H
# define BUILTINS_H

#include "struct.h"

// size_t		echo(char *cmd, char **args, char **envp);
void		builtin_exit(t_shell *shell, int exitcode);
int			builtin_env(char **envp);
#endif