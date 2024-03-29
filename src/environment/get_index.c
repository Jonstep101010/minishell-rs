/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_index.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:35:03 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:35:27 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"
#include <stdbool.h>

static size_t	get_key_len(const char *s)
{
	size_t	len;
	size_t	key_len;

	key_len = 0;
	len = ft_strlen(s);
	if (*s == '=')
		return (-1);
	while (key_len < len && s[key_len] != '=')
		key_len++;
	if (s[key_len] == '=' && key_len != 0)
		return (key_len);
	if (key_len == len)
		return (key_len);
	return (-1);
}

/**
 * @brief get the index of an environment variable
 *
 * @param env {"key=val", "key_two=val_two", NULL}
 * @param s can be key=val or key
 * @return int index of key=val or key, -1 if not found
 */
int	get_index_env(char *const *env, char const *substr)
{
	size_t	i;
	size_t	key_len;

	if (!env || !substr)
		return (-1);
	i = 0;
	key_len = get_key_len(substr);
	if (key_len <= 0)
		return (-1);
	while (env[i])
	{
		if (ft_strncmp(env[i], substr, key_len) == 0
			&& env[i][key_len] && env[i][key_len] == '=')
			return (i);
		i++;
	}
	return (-1);
}
