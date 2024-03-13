/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   executils.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: apeposhi <apeposhi@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/13 10:28:15 by apeposhi          #+#    #+#             */
/*   Updated: 2024/03/13 10:55:04 by apeposhi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "minishell.h"
#include "struct.h"
#include <string.h>

char	*ft_strmanip(char *path, char *f_cmd)
{
	char	*path_data;
	char	*cmd;

	path_data = ft_strjoin(path, "/");
	cmd = ft_strjoin(path_data, f_cmd);
	free(path_data);
	return (cmd);
}

char	*ft_getpath(char **env, char *f_cmd)
{
	char	*path;
	char	*tmp;
	char	*s_tmp;

 	s_tmp = NULL;
	path = ft_arrcmp(env, "PATH=");
	if (!path)
		return (NULL);
	path += 5;
	tmp = path;
	while (!access(path, X_OK)) {
		if (s_tmp)
			free(s_tmp);
		path = tmp;
		while (*tmp && *tmp != ':')
			tmp++;
		*tmp++ = '\0';
 		s_tmp = ft_strmanip(path, f_cmd);
	}
	return (s_tmp);
}

// function to check if it's the last command

// function to check if piped