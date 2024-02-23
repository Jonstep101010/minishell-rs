#ifndef COMMANDS_H
# define COMMANDS_H
# include "struct.h"
int							not_builtin(t_shell *shell, t_token *token);
int							builtin_cd(t_shell *shell, t_token *token);
int							export(t_shell *shell, t_token *token);
int							unset(t_shell *shell, t_token *token);
int							echo(t_shell *nullable, t_token *token);
int							builtin_exit(t_shell *shell, t_token *nullable);
int							builtin_env(t_shell *shell, t_token *token);
int							builtin_pwd(t_shell *shell, t_token *token);

typedef int					(*t_cmd_func_builtin)(t_shell *, t_token *);

typedef const struct s_func
{
	char					*name;
	t_cmd_func_builtin		cmd;
}	t_func;

static const struct s_func	g_cmds[] = {
{"echo", echo}, {"cd", builtin_cd},
{"pwd", builtin_pwd}, {"export", export},
{"unset", unset}, {"env", builtin_env},
{"exit", builtin_exit}, {NULL, NULL}};
#endif