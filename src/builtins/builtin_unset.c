/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   builtin_unset.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:25:00 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:25:01 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "environment.h"
#include "tokens.h"
#include "utils.h"
#include "struct.h"
#include "../../include/libft-rs/libft.h"

static bool	check_illegal_char(const char *str)
{
	while (*str)
	{
		if (ft_strchr("?=;$.:><[]()/+-{}*#@!^", *str))
			return (true);
		str++;
	}
	return (false);
}

static int	unset_internal(const char *const *args, char **env)
{
	int	index;

	while (*args)
	{
		if (!check_valid_key(*args) || check_illegal_char(*args))
		{
			eprint("unset: `%s': not a valid identifier", *args);
			return (1);
		}
		index = get_index_env(env, *args);
		if (index >= 0 && env[index])
			rm_str_arr(env, env[index]);
		args++;
	}
	return (0);
}

int	builtin_unset(t_shell *shell, t_token *token)
{
	int			status;
	const char	**args = (const char **)get_cmd_arr_token(token);

	if (!shell->env || !args || !*shell->env)
		return (0);
	if (!*args || !args[1] || !*args[1])
	{
		arr_free((char **)args);
		return (0);
	}
	status = unset_internal(args + 1, shell->env);
	arr_free((char **)args);
	return (status);
}
