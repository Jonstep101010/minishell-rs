/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   builtin_cd.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:13:42 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:15:39 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libutils.h"
#include "utils.h"
#include "struct.h"
#include "environment.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <err.h>
#include "str_utils.h"

/**
 * @brief change directory to path, update PWD and OLDPWD
 * @param path if no path is given, change to home
 * @param shell to update env/exit for exporting
 * @return int
 */
static int	changedir(const char *path, t_shell *shell)
{
	char	*pwd;
	char	*oldpwd;

	oldpwd = getcwd(NULL, 0);
	if (chdir(path) == -1)
	{
		eprint("cd: %s: %s", path, strerror(errno));
		return (free(oldpwd), -1);
	}
	pwd = getcwd(NULL, 0);
	if (!pwd)
	{
		eprint("cd: %s: %s", path, strerror(errno));
		return (free(oldpwd), -1);
	}
	export_env(shell, free_second_join("PWD=", pwd));
	export_env(shell, free_second_join("OLDPWD=", oldpwd));
	return (0);
}

/**
 * @brief change directory
 * @param cmd_args dir to change to
 * @param shell to update env
 * @return int exit code
 */
static int	cd_internal(const char **cmd_args, t_shell *shell)
{
	char	*path;
	char	*oldpwd;

	path = get_env(shell->env, "HOME");
	oldpwd = get_env(shell->env, "OLDPWD");
	if (!cmd_args[1] && !path)
		return (free_null(&oldpwd), eprint("cd: HOME not set"), 1);
	if (!cmd_args[1] && path)
		changedir(path, shell);
	else if (*cmd_args[1] == '~' && path)
	{
		path = free_first_join(path, cmd_args[1] + 1);
		if (changedir(path, shell) == -1)
			return (free(path), free(oldpwd), 1);
	}
	else if (*cmd_args[1] == '-' && oldpwd)
	{
		oldpwd = free_first_join(oldpwd, cmd_args[1] + 1);
		if (changedir(oldpwd, shell) == -1)
			return (free(path), free(oldpwd), 1);
	}
	else if (changedir(cmd_args[1], shell) == -1)
		return (free(path), free(oldpwd), 1);
	return (free(path), free(oldpwd), 0);
}

int	builtin_cd(t_shell *shell, t_token *token)
{
	int			status;
	const char	**command = (const char **)get_cmd_arr_token(token);

	status = cd_internal(command, shell);
	arr_free((char **)command);
	return (status);
}
