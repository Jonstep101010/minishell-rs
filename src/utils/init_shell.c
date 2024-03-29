/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   init_shell.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:20:04 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:20:18 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "struct.h"
#include "libft.h"
#include "utils.h"
#include "environment.h"
#include "arr_utils.h"
#include "utils.h"
#include "libutils.h"

static char	**init_env(char *const *envp)
{
	char	**env;
	char	*pwd;
	char	*path;

	env = append_str_arr(envp, "?=0");
	pwd = get_env(env, "PWD");
	if (!pwd && env)
	{
		pwd = getcwd(NULL, 0);
		if (!pwd)
			return (arr_free(env), NULL);
		env = append_str_arr_free(
				append_str_arr_free(env,
					free_second_join("PWD=", pwd)), ft_strdup("OLDPWD=''"));
	}
	else
		free_null(&pwd);
	path = get_env(env, "PATH");
	if (!path)
		env = append_str_arr_free(env,
				ft_strdup("PATH=/bin:/usr/bin:/sbin/:/usr/sbin"));
	else
		free_null(&path);
	return (env);
}

t_shell	*init_shell(char *const *envp)
{
	t_shell	*shell;

	shell = ft_calloc(1, sizeof(t_shell));
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
