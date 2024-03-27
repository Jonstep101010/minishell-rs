#include "lexer.h"
#include "libft.h"
#include <stdlib.h>

static enum e_lexer	check_quotes(const char *s, struct s_lexer *input)
{
	if (input->singlequotes == 1)
		return (LEXER_SINGLE_QUOTE);
	if (input->doublequotes == 1)
		return (LEXER_DOUBLE_QUOTE);
	if (input->singlequotes % 2 != 0 || input->doublequotes % 2 != 0)
		return (LEXER_UNBALANCED_QUOTES);
	if (input->singlequotes > 0 || input->doublequotes > 0)
		ignore_quotes(s, input);
	return (LEXER_SUCCESS);
}

t_lexer	*lexer_checks_basic(const char *s)
{
	t_lexer	*input;

	input = ft_calloc(sizeof(t_lexer), 1);
	input->lexer = LEXER_BEGIN;
	count_number(s, input);
	input->lexer = check_quotes(s, input);
	if (input->lexer != LEXER_SUCCESS)
		return (free(input->ignore), input);
	input->lexer = LEXER_BEGIN;
	if (input->pipes || input->redir_greater || input->redir_smaller)
		input->lexer = check_pipes_redirection(s, input);
	if (input->lexer == LEXER_BEGIN)
		input->lexer = LEXER_SUCCESS;
	return (free(input->ignore), input);
}
