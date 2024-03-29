/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_input.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:19:35 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:19:58 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"
#include "libutils.h"
#include "tokens.h"
#include "utils.h"

static char	*collect_as_ascii(char *readline_line)
{
	int		i;
	char	*collected_line;

	i = 0;
	collected_line = NULL;
	if (readline_line && !*readline_line)
	{
		return (free(readline_line), ft_strdup(""));
	}
	while (readline_line && readline_line[i])
	{
		if (ft_isascii(readline_line[i]))
		{
			collected_line = append_char_str(collected_line, readline_line[i]);
			if (!collected_line)
				return (free(readline_line), NULL);
		}
		i++;
	}
	if (readline_line)
		free(readline_line);
	return (collected_line);
}

char	*get_input(char *rl_prompt)
{
	static char	*trim;
	char		*line;

	if (!rl_prompt)
		return (free(trim), NULL);
	line = collect_as_ascii(rl_prompt);
	if (!line)
		return (NULL);
	trim = ft_strtrim(line, WHITESPACE);
	free(line);
	if (!trim)
		return (NULL);
	return (trim);
}
