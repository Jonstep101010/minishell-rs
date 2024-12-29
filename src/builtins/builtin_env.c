/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   builtin_env.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:16:26 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:16:56 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "struct.h"
#include "tokens.h"
#include <stdio.h>

/**
 * @brief print all environment variables
 */
int	builtin_env(t_shell *shell, t_token *token)
{
	const char *const	*env = (const char *const *)shell->env;

	(void)token;
	while (*env)
	{
		printf("%s\n", *env);
		env++;
	}
	return (0);
}
