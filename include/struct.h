/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   struct.h                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:03:19 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/30 11:20:31 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef STRUCT_H
# define STRUCT_H

# include "tokens.h"
# include <stdbool.h>
# include <stdint.h>
# include <termios.h>

typedef struct s_shell
{
	uint8_t			exit_status;
	char			**env;
	t_token			*token;
	size_t			token_len;
	struct termios	p_termios;
}	t_shell;

void	update_exit_status(t_shell *shell, int status);

#endif