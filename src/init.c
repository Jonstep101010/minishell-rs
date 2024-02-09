#include "struct.h"
#include "libft.h"
#include "utils.h"
#include "environment.h"

char	**init_env(const char **envp)
{
	char	**env;

	if (!envp)
		return (NULL);
	env = append_str_arr(envp, "$?=0");
	if (!env)
		return (NULL);
	return (env);
}

t_shell	*init_shell(const char **envp)
{
	t_shell	*shell;

	shell = (t_shell *)ft_calloc(1, sizeof(t_shell));
	if (!shell)
		return (NULL);
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
	new_status = ft_strjoin("$?=", status_str);
	free(status_str);
	if (!new_status)
		return ;
	shell->tmp_arr = export_var(shell->owned_envp, new_status);
	if (!shell->tmp_arr)
	{
		free(status_str);
		free(new_status);
		return ;
	}
}
