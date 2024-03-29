/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.h                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:02:04 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 19:09:41 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef LEXER_H
# define LEXER_H

# include <stddef.h>
# include <stdbool.h>

# define LEXER_SUCCESS 0
# define ERR_TOKEN "syntax error near unexpected token "

struct s_check_pipes
{
	int		flag_redir;
	int		flag_word;
	size_t	i;
	bool	ignore;
};

typedef struct s_lexer
{
	int		singlequotes;
	int		doublequotes;
	int		open_curly_brackets;
	int		close_curly_brackets;
	int		open_square_brackets;
	int		close_square_brackets;
	int		open_parentheses;
	int		close_parentheses;
	int		redir_greater;
	int		redir_smaller;
	int		pipes;
	bool	*ignore;
	size_t	len;
	int		lexer;
	bool	result;
}	t_lexer;


int				ignore_quotes(const char *s, struct s_lexer *input);
int				check_pipes_redirection(const char *s, t_lexer *input);
void			count_number(const char *s, struct s_lexer *input);

bool			*bool_arr_zeroing(size_t len);
void			range_ignore(const char *s, bool *ignore, unsigned char c);

#endif // LEXER_H