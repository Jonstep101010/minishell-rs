/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   exit_free.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:19:22 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:19:23 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "arr_utils.h"
#include "struct.h"
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include "utils.h"

void	exit_free(t_shell *shell, int exit_code)
{
	if (shell->env)
		arr_free(shell->env);
	destroy_all_tokens(shell);
	free(shell);
	exit(exit_code);
}

void	exit_error(t_shell *shell, char *error_elem)
{
	char	*error;

	error = strerror(errno);
	if (error_elem)
		eprint("%s: %s", error_elem, error);
	if (shell->env)
		arr_free(shell->env);
	destroy_all_tokens(shell);
	free(shell);
	exit(errno);
}
