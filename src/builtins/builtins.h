#ifndef BUILTINS_H
# define BUILTINS_H

#include "struct.h"

int		builtin_cd(char **cmd_args, char **envp);
char	*occurs_exclusively(const char *, const char *);

int		export(t_shell *shell, t_token *token);
int		unset(const char **args, char **envp);
size_t	echo(char *cmd, char **args, char **envp);
void	builtin_exit(t_shell *shell, int exitcode);
int		builtin_env(char **envp);
int		builtin_pwd(const char **envp);

void	add_pipe_split_as_tokens(char **pipe_split, t_shell *shell);
void	convert_split_token_string_array_to_tokens(t_shell *shell);
void	convert_tokens_to_string_array(t_token *token);
void	destroy_all_tokens(t_shell *shell);

#endif