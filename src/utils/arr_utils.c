/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   arr_utils.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:14:46 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:16:08 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"
#include <stddef.h>
#include "utils.h"
#include "arr_utils.h"

void	rm_str_arr(char **arr, const char *s)
{
	size_t	i;
	size_t	len;

	if (!arr || !s)
		return ;
	i = 0;
	len = arr_len(arr);
	while (arr[i])
	{
		if (equal(arr[i], s))
		{
			free(arr[i]);
			while (i < len)
			{
				arr[i] = arr[i + 1];
				i++;
			}
			return ;
		}
		i++;
	}
}
