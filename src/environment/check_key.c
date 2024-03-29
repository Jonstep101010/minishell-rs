/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   check_key.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:32:12 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:32:13 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"
#include <stdbool.h>

bool	check_valid_key(const char *s)
{
	int	i;

	i = 0;
	if (*s == '=' || !(ft_isalpha(*s) || *s == '_'))
		return (false);
	while (s[i] && s[i] != '=')
	{
		if (ft_isalnum(s[i]) || s[i] == '_')
			i++;
		else
			return (false);
	}
	if (s[i] == '=' && s[i + 1] == '=')
		return (false);
	return (true);
}
