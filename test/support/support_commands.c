#ifndef SUPPORT_COMMANDS_H
# define SUPPORT_COMMANDS_H
# include "builtin_pwd.c"
# include "builtin_env.c"
# include "builtin_cd.c"
# include "builtin_unset.c"
# include "builtin_export.c"
# include "builtin_echo.c"
# include "get_index.c"
# include "check_key.c"
# include "get_env.c"
// mock function to avoid dependencies for testing
int	not_builtin(t_shell *shell, t_token *token)
{
	(void)shell;
	(void)token;
	return (0);
}
# include "builtin_exit.c"
# include "export_env.c"
# include "error.c"
#endif
