#include "lexer.h"
#include "libft.h"
#include <stdlib.h>
#include "utils.h"

int	check_quotes(const char *s, struct s_lexer *input)
{
	if (input->singlequotes == 1)
		return (eprint_single("syntax error near unexpected token '''\n"), 1);
	if (input->doublequotes == 1)
		return (eprint_single("syntax error near unexpected token '\"'\n"), 1);
	if (input->singlequotes % 2 != 0 || input->doublequotes % 2 != 0)
		return (eprint_single("error: quotes not closed"), 1);
	if (input->singlequotes > 0 || input->doublequotes > 0)
		ignore_quotes(s, input);
	return (LEXER_SUCCESS);
}

t_lexer	*lexer_checks_basic(const char *s)
{
	t_lexer	*input;

	input = ft_calloc(sizeof(t_lexer), 1);
	count_number(s, input);
	input->lexer = check_quotes(s, input);
	if (input->lexer != 0)
		return (free(input->ignore), input);
	if (input->pipes || input->redir_greater || input->redir_smaller)
	{
		input->lexer = check_pipes_redirection(s, input);
		if (input->lexer != 0)
			return (free(input->ignore), input);
	}
	input->result = true;
	return (free(input->ignore), input);
}
