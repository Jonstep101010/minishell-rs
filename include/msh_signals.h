/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   msh_signals.h                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:02:42 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:02:50 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef MSH_SIGNALS_H
# define MSH_SIGNALS_H

# include <termios.h>
# include <stdbool.h>

extern int	g_ctrl_c;

void	check_signals(struct termios *p_termios);
void	check_signals_child(struct termios *p_termios_child);
#endif