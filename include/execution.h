#ifndef EXECUTION_H
# define EXECUTION_H
# include "struct.h"

void	execve_fail(t_shell *shell, char *cmd);

void	execute_pipes(t_shell *shell, int **pipes, int token_count);
#endif