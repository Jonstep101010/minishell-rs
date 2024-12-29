/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   utils.h                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:05:50 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:17:48 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef UTILS_H
# define UTILS_H

# include "../../include/libutils-rs/include/arr_utils.h"
# include "tokens.h"
# include <stddef.h>
# include <stdbool.h>
# include "struct.h"

char	*equal(const char *expected, const char *actual);
void	rm_str_arr(char **arr, const char *s);

/**
 * @brief "minishell: " + fmt on stderr
 */
void	eprint(const char *fmt, ...);

/**
 * @brief fmt on stderr
 */
void	eprint_single(const char *fmt, ...);
void	exit_free(t_shell *shell, int exit_code);
void	exit_error(t_shell *shell, char *error_elem);

#endif