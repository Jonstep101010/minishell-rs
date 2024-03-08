#include "libft.h"
#include "environment.h"
#include <stdbool.h>

// export VARNAME="value"
// export VARNAME='value'
// export VARNAME=value
// guaranteed to not have unbalanced quotes at this point

#include "utils.h"
// pass in address of string in env
static void	update_var(char **env, char *key_val)
{
	if (!key_val)
		return ;
	free(*env);
	*env = key_val;
}

// need to assign always
// input like key=val -> heap allocated!
char	**export_var(char **env, const char *key_val)
{
	int		index;

	if (!env || !key_val || !*key_val)
		return (NULL);
	index = find_key_env(env, key_val, get_key_len);
	if (index == -1)
		return (append_str_arr_free(env, ft_strdup(key_val)));
	update_var(&env[index], ft_strdup(key_val));
	return (env);
}

#include "commands.h"

/**
 * @brief directly replaces strings, key_val needs to be heap allocated
 * @warning will
 * @param shell
 * @param key_val
 */
void	export_to_shell(t_shell *shell, char *key_val)
{
	int	index;

	if (!key_val || !*key_val)
		return ;// @follow-up handle error?
	if (!shell || !shell->owned_envp || !*shell->owned_envp
		|| !**(shell->owned_envp))
	{
		free(key_val);
		shell->exit_status = 1;
		eprint("fatal: invalid memory!\n");
		builtin_exit(shell, NULL);
	}
	index = find_key_env(shell->owned_envp, key_val, get_key_len);
	if (index == -1)
		shell->owned_envp = append_str_arr_free(shell->owned_envp, key_val);
	else
		update_var(&shell->owned_envp[index], key_val);
	if (!shell->owned_envp)
	{
		free(key_val);
		shell->exit_status = 1;
		eprint("fatal: environment invalidated\n");
		builtin_exit(shell, NULL);
	}
}

