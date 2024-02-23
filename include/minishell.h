#ifndef MINISHELL_H
# define MINISHELL_H
/* helper functions to prototype*/
#include "struct.h"
int while_d(const char *s, int (*)(int), int is_true, int *index);
int while_i(const char *s, int (*)(int), int is_true, int *index);
int	while_not_i(const char *s, int (*)(int), char c, int *index);
int	while_is_i(const char *s, char c, int *index);

// initialize shell
char	**init_env(const char **envp);
t_shell	*init_shell(const char **envp);
void	update_exit_status(t_shell *shell, int status);

// parsing / tokenization
void	deploy_tokens(t_shell *shell);
void	destroy_all_tokens(t_shell *shell);

int		builtin(t_shell *shell, t_token *token);
void	execute_commands(t_shell *shell, t_token *token);

#endif
