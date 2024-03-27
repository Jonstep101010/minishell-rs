#ifndef MINISHELL_H
# define MINISHELL_H
/* helper functions to prototype*/
# include "struct.h"
int		lexer(t_shell *shell, const char *trimmed_line);
// initialize shell
void	update_exit_status(t_shell *shell, int status);

// parsing / tokenization
void	deploy_tokens(t_shell *shell);
void	execute_commands(t_shell *shell, t_token *token);

#endif
