#include "lexer.h"
#include <stdlib.h>

// @audit-info error  "cmd "'" hello" or any other unbalanced delimiters
// @follow-up free in caller and just get ref to struct?
t_lexer	lexer_checks_basic(const char *s)
{
	if (!s || !*s)
		return (LEXER_NULL);
	struct s_lexer	input;
	input.lexer = LEXER_NULL;
	count_number(s, &input);
	if (input.singlequotes > 0 || input.doublequotes > 0)
		ignore_quotes(s, &input);
	if (check_pipes_redirection(s, &input) != LEXER_SUCCESS && check_against_ignore(s, &input) != LEXER_SUCCESS)
		return (free(input.ignore), input.lexer);
	if (check_brackets_quotes(&input) != LEXER_SUCCESS && check_against_ignore(s, &input) != LEXER_SUCCESS)
		return (free(input.ignore), input.lexer);
	if (input.singlequotes % 2 == 0 && (input.open_curly_brackets == input.close_curly_brackets) && (input.open_curly_brackets + input.close_curly_brackets) % 2 == 0)
		return (free(input.ignore), LEXER_SUCCESS);
	if (input.singlequotes > 0 && input.doublequotes % 2 == 0 && check_against_ignore(s, &input) == LEXER_SUCCESS)
		return (free(input.ignore), LEXER_SUCCESS);
	return (free(input.ignore), LEXER_NULL);
}
