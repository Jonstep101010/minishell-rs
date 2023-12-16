#include "minishell.h"
#include <stdbool.h>

static int	str_cchr(char *s, char c)
{
	int	i;
	int	count;

	i = 0;
	count = 0;
	while (s[i])
	{
		if (s[i] == c)
			count++;
		i++;
	}
	return (count);
}

void	count_number(char *s, struct s_lexer *input)
{
	*input = (struct s_lexer){
		.singlequotes = str_cchr(s, '\''),
		.doublequotes = str_cchr(s, '\"'),
		.open_curly_brackets = str_cchr(s, '{'),
		.close_curly_brackets = str_cchr(s, '}'),
		.open_square_brackets = str_cchr(s, '['),
		.close_square_brackets = str_cchr(s, ']'),
		.open_parentheses = str_cchr(s, '('),
		.close_parentheses = str_cchr(s, ')'),
	};
}

bool check_event_not_found(char *s)
{
	int	i;

	i = 0;
	while (s[i])
	{
		if (s[i] == '!')
		{
			if (s[i + 1] == '\"' || s[i + 1] == '\'')
				return (true);
		}
		i++;
	}
	return (false);
}

// make return value an enum
enum e_lexer	lexer(char *s)
{
	if (!s)
		return (LEXER_NULL);
	struct s_lexer	input;
	count_number(s, &input);

	// even_number_check(s);
	if (input.singlequotes % 2 != 0 || input.doublequotes % 2 != 0)
		return (LEXER_UNBALANCED_QUOTES);
	// amount of brackets needs to be even
	if (input.open_curly_brackets != input.close_curly_brackets || input.
		open_square_brackets != input.close_square_brackets || input.open_parentheses != input.close_parentheses)
		return (LEXER_UNBALANCED_BRACKETS);
	// printf("not working\n");
	if (input.open_curly_brackets + input.close_curly_brackets % 2 != 0 || input.open_square_brackets + input.close_square_brackets % 2 != 0 || input.open_parentheses + input.close_parentheses % 2 != 0)
		return (LEXER_UNBALANCED_BRACKETS);
	// check for this kind of input: echo \"Hello, World!\\\"
	if (str_cchr(s, '\\') % 2 != 0)
		return (LEXER_UNBALANCED_QUOTES);
	if (check_event_not_found(s))
		return (LEXER_UNBALANCED_QUOTES);
	if (str_cchr(s, '\\') == 0 || str_cchr(s, '\\') % 2 == 0)
		return (LEXER_UNBALANCED_QUOTES);
	if (str_cchr(s, '\'') % 2 == 0 && (str_cchr(s, '{') == str_cchr(s, '}')) && (str_cchr(s, '{') + str_cchr(s, '}')) % 2 == 0)
		return (LEXER_SUCCESS);
	return (LEXER_NULL);
}
