/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   builtin_export.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:24:46 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:24:47 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "struct.h"
#include "environment.h"
#include <stdio.h>
#include <unistd.h>
#include "tokens.h"
#include "str_utils.h"
#include "utils.h"
#include "commands.h"
#include "libft.h"

static int	declare_x_env_var(char *const *env)
{
	while (*env)
	{
		printf("declare -x %s\n", *env);
		env++;
	}
	return (0);
}

int	builtin_export(t_shell *shell, t_token *token)
{
	const char	**command = (const char **)get_cmd_arr_token(token);
	size_t		i;

	i = 1;
	if (!command || !command[i])
		return (arr_free((char **)command), declare_x_env_var(shell->env));
	while (command[i])
	{
		if (!check_valid_key(command[i]))
		{
			eprint("export: `%s': not a valid identifier", command[i]);
			arr_free((char **)command);
			return (1);
		}
		if (str_cchr(command[i], '=') >= 1)
			export_env(shell, ft_strdup(command[i]));
		i++;
	}
	arr_free((char **)command);
	if (i > 1)
		return (0);
	return (1);
}
