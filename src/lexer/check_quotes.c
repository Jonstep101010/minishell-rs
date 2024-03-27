#include "lexer.h"
#include "libft.h"

enum e_lexer	ignore_quotes(const char *s, struct s_lexer *input)
{
	if (!s || !input)
		return (LEXER_NULL);
	if (input->singlequotes % 2 != 0
		&& (input->doublequotes % 2 == 0 || input->doublequotes == 0))
		;
	else if (!(input->doublequotes % 2 == 0 || input->singlequotes % 2 == 0))
		return (LEXER_UNBALANCED_QUOTES);
	input->ignore = bool_arr_zeroing(ft_strlen(s));
	range_ignore(s, input->ignore, '"');
	range_ignore(s, input->ignore, '\'');
	return (LEXER_SUCCESS);
}

enum e_lexer	check_against_ignore(const char *s, struct s_lexer *input)
{
	int	i;

	if (!s)
		return (LEXER_NULL);
	if (input->ignore == NULL)
		return (LEXER_NULL);
	i = 0;
	while (i < (int)input->len)
	{
		if (input->ignore[i] == false)
		{
			if (s[i] == '|')
				return (LEXER_PIPES);
			if (s[i] == '<' || s[i] == '>')
				return (LEXER_REDIRECTION);
			if (s[i] == '\'' || s[i] == '"')
				return (LEXER_UNBALANCED_QUOTES);
		}
		i++;
	}
	return (LEXER_SUCCESS);
}
