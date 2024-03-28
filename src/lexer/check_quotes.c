#include "lexer.h"
#include "libft.h"

int ignore_quotes(const char *s, struct s_lexer *input)
{
	if (!s || !input)
		return (1);
	input->ignore = bool_arr_zeroing(ft_strlen(s));
	range_ignore(s, input->ignore, '"');
	range_ignore(s, input->ignore, '\'');
	return (LEXER_SUCCESS);
}

