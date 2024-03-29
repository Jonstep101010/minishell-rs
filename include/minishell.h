/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   minishell.h                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:02:19 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:02:20 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef MINISHELL_H
# define MINISHELL_H
# include "struct.h"

int		lexer(t_shell *shell, const char *trimmed_line);

// entrypoint to execution
void	execute_commands(t_shell *shell, t_token *token);

// initialize shell
t_shell	*init_shell(char *const *envp);
char	*get_input(char *rl_prompt);
#endif
