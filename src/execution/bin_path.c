#include "environment.h"
#include "libft.h"
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include "arr_utils.h"
#include "libutils.h"

static uint8_t	find_bin(const char **paths, const char *binprefix
						, char **binpath_buf)
{
	if (!binprefix)
		return (1);
	while (paths && *paths)
	{
		*binpath_buf = ft_strjoin(*paths, binprefix);
		if (binpath_buf && !*binpath_buf)
			return (EXIT_FAILURE);
		if (access(*binpath_buf, F_OK) == 0)
			return (0);
		free_null(binpath_buf);
		paths++;
	}
	return (127);
}

static uint8_t	get_bin(const char **paths, const char *bin, char **binpath_buf)
{
	const char	*bin_prefix = ft_strjoin("/", bin);
	const char	*syspaths[] = {"/usr/local/bin", "/usr/bin",
		"/bin", "/sbin", "/usr/sbin", NULL};
	uint8_t		status;

	if (!bin_prefix)
		return (arr_free((char **)paths), 1);
	status = find_bin(paths, bin_prefix, binpath_buf);
	arr_free((char **)paths);
	if (status == 127)
		status = find_bin(syspaths, bin_prefix, binpath_buf);
	free((char *)bin_prefix);
	if (status == 0 && access(*binpath_buf, X_OK) == -1)
		return (126);
	return (status);
}

/**
 * @brief Get the path to binary
 *
 * @param env shell->env
 * @param bin
 * @param binpath_buf &(token->bin)
 * @return uint8_t 0 on success, 1 on error
 * \return 126 if not executable, 127 if not found
 */
uint8_t	set_binpath(char *const *env, const char *bin, char **binpath_buf)
{
	char		*path;
	const char	**paths;

	if (!bin || !*bin)
		return (EXIT_FAILURE);
	if (*bin == '~' && !*(bin + 1))
		return (126);
	if (*bin == '.' || *bin == '/')
	{
		*binpath_buf = ft_strdup(bin);
		if (!*binpath_buf)
			return (EXIT_FAILURE);
		if (access(*binpath_buf, F_OK) == -1)
			return (127);
		if (access(*binpath_buf, X_OK) == -1)
			return (126);
		return (0);
	}
	path = get_env(env, "PATH");
	paths = (const char **)ft_split(path, ':');
	free(path);
	return (get_bin(paths, bin, binpath_buf));
}
