#include "lexer.h"
#include "libft.h"

void	count_number(char *s, struct s_lexer *input)
{
	*input = (struct s_lexer){
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
		.lexer = LEXER_NULL,
		.len = (int)ft_strlen(s),
	};
}

t_lexer	check_brackets_quotes(struct s_lexer *input)
{
	input->lexer = LEXER_UNBALANCED_QUOTES;
	if (input->singlequotes % 2 != 0 || input->doublequotes % 2 != 0)
		return (LEXER_UNBALANCED_QUOTES);
	input->lexer = LEXER_UNBALANCED_BRACKETS;
	if (input->open_curly_brackets != input->close_curly_brackets || input->
		open_square_brackets != input->close_square_brackets || input->open_parentheses != input->close_parentheses)
		return (LEXER_UNBALANCED_BRACKETS);
	if ((input->open_curly_brackets + input->close_curly_brackets) % 2 != 0 ||(input->open_square_brackets + input->close_square_brackets) % 2 != 0 || (input->open_parentheses + input->close_parentheses) % 2 != 0)
		return (LEXER_UNBALANCED_BRACKETS);
	return (LEXER_SUCCESS);
}
