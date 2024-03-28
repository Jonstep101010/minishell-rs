#include "lexer.h"
#include "libft.h"
#include "str_utils.h"

void	count_number(const char *s, t_lexer *input)
{
	const t_lexer data = {
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
