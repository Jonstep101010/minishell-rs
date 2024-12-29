/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   environment.h                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 17:59:17 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:00:18 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef ENVIRONMENT_H
# define ENVIRONMENT_H

# include <stdbool.h>
# include "struct.h"

int		get_index_env(char *const *env, char const *substr);
char	*get_env(char *const *env, char const *key);

bool	check_valid_key(const char *s);

void	export_env(t_shell *shell, char *key_val);

char	*expander(char const *input_expander, char *const *env);

#endif