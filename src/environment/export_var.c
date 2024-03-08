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
		return ((void)eprint("export: malloc fail creating key_val\n"));
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
		shell->exit_status = 1;
		eprint("fatal: environment invalidated\n");
		builtin_exit(shell, NULL);
	}
}

void	update_exit_status(t_shell *shell, int status)
{
	export_to_shell(shell,
		free_second_join("?=", ft_itoa(status)));
	shell->exit_status = status;
}
