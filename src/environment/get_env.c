/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_env.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:34:53 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:34:54 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"
#include "environment.h"

/**
 * @brief get an environment variable
 *
 * @param env {"key=val", "key_two=val_two", NULL}
 * @param key to find value of
 * @return char* malloced value, NULL if not found
 */
char	*get_env(char *const *env, char const *key)
{
	int			index;

	if (!env || !key)
		return (NULL);
	index = get_index_env(env, key);
	if (index != -1 && env[index])
	{
		return (ft_substr(env[index], ft_strlen(key) + 1,
				ft_strlen(env[index]) - ft_strlen(key) + 1)
		);
	}
	return (NULL);
}
