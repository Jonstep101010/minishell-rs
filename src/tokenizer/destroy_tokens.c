/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   destroy_tokens.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:07:10 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/30 11:29:11 by jschwabe         ###   ########.fr       */
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
	while (token && i < shell->token_len)
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
		if (token[i].bin)
			free_null(&(token[i].bin));
		i++;
	}
	free_null(&shell->token);
	shell->token_len = 0;
}
