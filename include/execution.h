#ifndef EXECUTION_H
# define EXECUTION_H
# include "struct.h"

void	do_redirections(t_token *token);
void	do_heredocs(t_token *token, const int *target, char **env);
void	heredoc_nopipe(t_token *token, char **env);

void	execve_fail(t_shell *shell, char *cmd);

void	execute_pipes(t_shell *shell, int **pipes, int token_count);
#endif