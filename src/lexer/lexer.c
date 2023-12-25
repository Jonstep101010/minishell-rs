#include "libft.h"
#include "lexer.h"
#include "struct.h"
#include <stdbool.h>

t_lexer	check_brackets_quotes(const char *s, struct s_lexer *input)
{
	(void)s;
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


// "this is my input "ignore" 't' 'this' "is" 'a' "test" 'string'""
// "                 00000000 000 000000 0000 000 000000 000000000"

// handle input like this "this is a cmd "'" hello" (accept in quotes)
// make other checks use the ignore array
t_lexer	lexer(char *s)
{
	if (!s || !*s)
		return (LEXER_NULL);
	// char *s = ft_strtrim(str, " ");
	struct s_lexer	input;
	count_number(s, &input);
	// range_ignore(s, &input.ignore, '\'');
	if (input.singlequotes > 0 || input.doublequotes > 0)
		check_quotes_ignore(s, &input);

	if (check_pipes_redirection(s, &input) != LEXER_SUCCESS)
		return (input.lexer);
	if (check_brackets_quotes(s, &input) != LEXER_SUCCESS)
		return (input.lexer);
	if (input.singlequotes % 2 == 0 && (input.open_curly_brackets == input.close_curly_brackets) && (input.open_curly_brackets + input.close_curly_brackets) % 2 == 0)
		return (LEXER_SUCCESS);
	return (LEXER_NULL);
}
