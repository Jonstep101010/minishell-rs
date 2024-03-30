/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   exec_bin.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:37:32 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/30 10:14:26 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "commands.h"
#include "libft.h"
#include "struct.h"
#include "tokens.h"
#include "utils.h"
#include <errno.h>
#include <stdbool.h>
#include <string.h>
#include <sys/param.h>
#include <sys/wait.h>
#include <unistd.h>
#include "execution.h"

static void	execve_fail(t_shell *shell, char *cmd)
{
	eprint("%s: %s", cmd, strerror(errno));
	if (shell->env)
		arr_free(shell->env);
	destroy_all_tokens(shell);
	free(shell);
	exit(errno);
}

uint8_t	set_binpath(char *const *env, const char *bin, char **binpath_buf);

/**
 * @brief is called for each token (not builtin)
 */
int	exec_bin(t_shell *shell, t_token *token)
{
	int			access_status;
	const char	**command = (const char **)get_cmd_arr_token(token);

	if (!command)
		exit_free(shell, 0);
	access_status = set_binpath(shell->env, *command, &(token->bin));
	if (access_status == 1 || access_status == 2
		|| access_status == 126 || access_status == 127)
	{
		if (access_status == 126 && !ft_strchr("~/", **command))
			eprint("%s: %s", *command, strerror(errno));
		if (access_status == 127)
			eprint("%s: command not found", *command);
		arr_free((char **)command);
		exit_free(shell, access_status);
	}
	if (execve(token->bin, (char **)command, shell->env) == -1)
	{
		arr_free((char **)command);
		execve_fail(shell, token->bin);
	}
	arr_free((char **)command);
	exit_free(shell, 0);
	return (0);
}
