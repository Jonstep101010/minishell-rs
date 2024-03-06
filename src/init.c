#include "struct.h"
#include "libft.h"
#include "utils.h"
#include "environment.h"
#include "arr_utils.h"
#include "utils.h"
#include "libutils.h"

char	**init_env(char *const *envp)
{
	char	**env;
	// char	**tmp_arr;
	char	*PWD;
	// char	*tmp;

	env = append_str_arr(envp, "?=0");
	PWD = get_env_var(env, "PWD");
	if (!PWD && env)
	{
		PWD = getcwd(NULL, 0);
		if (!PWD)
			return (arr_free(env), NULL);
		env = append_str_arr_free(append_str_arr_free(env, ft_strjoin("PWD=", PWD)), ft_strdup("OLDPWD=''"));
	}
	else
		free_null(&PWD);
	return (env);
}

t_shell	*init_shell(char *const *envp)
{
	t_shell	*shell;

	shell = (t_shell *)ft_calloc(1, sizeof(t_shell));
	if (!shell)
		exit(1);
	shell->p_termios = (struct termios){0};
	shell->owned_envp = init_env(envp);
	if (!shell->owned_envp)
	{
		free(shell);
		return (NULL);
	}
	return (shell);
}

void	update_exit_status(t_shell *shell, int status)
{
	char	*status_str;
	char	*new_status;

	status_str = ft_itoa(status);
	if (!status_str)
		return ;
	new_status = ft_strjoin("?=", status_str);
	free(status_str);
	if (!new_status)
		return ;
	shell->tmp_arr = export_var(shell->owned_envp, new_status);
	free(new_status);
	if (!shell->tmp_arr)
	{
		free(status_str);
		return ;
	}
	shell->exit_status = status;
	shell->owned_envp = shell->tmp_arr;
}
