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
		free_null(&PWD);
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
	shell->env = init_env(envp);
	if (!shell->env)
	{
		free(shell);
		return (NULL);
	}
	return (shell);
}
