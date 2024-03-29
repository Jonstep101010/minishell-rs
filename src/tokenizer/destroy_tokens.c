/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   destroy_tokens.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:07:10 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:10:20 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "struct.h"
#include "tokens.h"
#include "libutils.h"

void	destroy_all_tokens(t_shell *shell)
{
	t_token	*token;
	size_t	i;
	size_t	ii;

	token = shell->token;
	i = 0;
	while (token && shell->token->split_pipes && token[i].split_pipes)
	{
		if (token[i].cmd_args)
		{
			ii = 0;
			while (token[i].cmd_args[ii].elem)
			{
				free_null(&token[i].cmd_args[ii].elem);
				ii++;
			}
			free_null(&token[i].cmd_args);
		}
		free_null(&(token[i].split_pipes));
		if (token[i].bin)
			free_null(&(token[i].bin));
		i++;
	}
	free_null(&shell->token);
}
