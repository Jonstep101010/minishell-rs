/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer_support.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 19:52:04 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 19:52:35 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "lexer.h"
#include "../../include/libft-rs/libft.h"
#include "../../include/libutils-rs/include/str_utils.h"

void	count_number(const char *s, t_lexer *input)
{
	const t_lexer	data = {
		.singlequotes = str_cchr(s, '\''),
		.doublequotes = str_cchr(s, '"'),
		.open_curly_brackets = str_cchr(s, '{'),
		.close_curly_brackets = str_cchr(s, '}'),
		.open_square_brackets = str_cchr(s, '['),
		.close_square_brackets = str_cchr(s, ']'),
		.open_parentheses = str_cchr(s, '('),
		.close_parentheses = str_cchr(s, ')'),
		.redir_greater = str_cchr(s, '>'),
		.redir_smaller = str_cchr(s, '<'),
		.pipes = str_cchr(s, '|'),
		.ignore = NULL,
		.lexer = 0,
		.result = false,
		.len = (int)ft_strlen(s),
	};

	ft_memcpy(input, &data, sizeof(t_lexer));
}
