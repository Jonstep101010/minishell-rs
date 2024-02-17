#include "utils.h"
#include "environment.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <err.h>

/**
 * @brief change directory to path, update PWD and OLDPWD
 * @audit uses dprintf
 * @param path if no path is given, change to home
 * @param envp environment
 * @return int
 */
static int	changedir(char *path, char **envp)
{
	char	*pwd;
	char	*oldpwd;

	oldpwd = getcwd(NULL, 0);
	if (chdir(path) == -1)
	{
		dprintf(STDERR_FILENO, "minishell: cd: %s: %s\n", path, strerror(errno));
		return (free(oldpwd), -1);
	}
	pwd = getcwd(NULL, 0);
	if (!pwd)
	{
		dprintf(STDERR_FILENO, "minishell: cd: %s: %s\n", path, strerror(errno));
		return (free(oldpwd), -1);
	}
	update_variable(envp, "PWD", pwd);
	update_variable(envp, "OLDPWD", oldpwd);
	return (free(oldpwd), free(pwd), 0);
}

/**
 * @brief change directory
 * @audit uses dprintf
 * @param cmd_args dir to change to
 * @param envp environment
 * @return int exit code
 */
int	builtin_cd(char **cmd_args, char **envp)
{
	char	*path;
	char	*oldpwd;

	path = get_var_val((const char **)envp, "HOME");
	oldpwd = get_var_val((const char **)envp, "OLDPWD");
	if (!cmd_args[1] && !path)
		return (dprintf(STDERR_FILENO, "minishell: cd: HOME not set\n"), 1);
	if (!cmd_args[1] && path)
		changedir(path, envp);
	else if (*cmd_args[1] == '~' && path)
	{
		path = free_first_join(path, cmd_args[1] + 1);
		if (changedir(path, envp) == -1)
			return (free(path), free(oldpwd), 1);
	}
	else if (*cmd_args[1] == '-' && oldpwd)
	{
		oldpwd = free_first_join(oldpwd, cmd_args[1] + 1);
		if (changedir(oldpwd, envp) == -1)
			return (free(path), free(oldpwd), 1);
	}
	else if (changedir(cmd_args[1], envp) == -1)
		return (free(path), free(oldpwd), 1);
	return (free(path), free(oldpwd), 0);
}
