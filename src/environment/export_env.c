/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   export_env.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:34:41 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:34:42 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"
#include "environment.h"
#include <stdbool.h>
#include "utils.h"
#include "commands.h"
#include "str_utils.h"

// export VARNAME="value"
// export VARNAME='value'
// export VARNAME=value
// guaranteed to not have unbalanced quotes at this point

// pass in address of string in env
static void	update_var(char **env, char *key_val)
{
	if (!key_val || !env || !*env || !*key_val)
		return ;
	free(*env);
	*env = key_val;
}

/**
 * @brief directly replaces strings, key_val needs to be heap allocated
 * @warning will
 * @param shell
 * @param key_val
 */
void	export_env(t_shell *shell, char *key_val)
{
	int	index;

	if (!key_val || !*key_val)
		return ((void)eprint("export: malloc fail creating key_val\n"));
	if (!shell || !shell->env || !*shell->env
		|| !**(shell->env))
	{
		free(key_val);
		shell->exit_status = 1;
		eprint("fatal: invalid memory!\n");
		builtin_exit(shell, NULL);
	}
	index = get_index_env(shell->env, key_val);
	if (index == -1)
		shell->env = append_str_arr_free(shell->env, key_val);
	else
		update_var(&shell->env[index], key_val);
	if (!shell->env)
	{
		shell->exit_status = 1;
		eprint("fatal: environment invalidated\n");
		builtin_exit(shell, NULL);
	}
}

void	update_exit_status(t_shell *shell, int status)
{
	export_env(shell,
		free_second_join("?=", ft_itoa(status)));
	shell->exit_status = status;
}
