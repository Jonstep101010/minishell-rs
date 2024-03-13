/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   execution.h                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: apeposhi <apeposhi@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/13 10:55:53 by apeposhi          #+#    #+#             */
/*   Updated: 2024/03/13 11:00:30 by apeposhi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef EXECUTION_H
# define EXECUTION_H

# include "minishell.h"
# include <stddef.h>

char	*ft_strmanip(char *path, char *f_cmd);
char	*ft_getpath(char **env, char *f_cmd);
void	execute(t_shell *shell, int tmp);

#endif