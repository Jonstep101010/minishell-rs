#include "lexer.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <unistd.h>
#include "minishell.h"
#include "tokens.h"
#include <sys/wait.h>

t_lexer	*lexer_checks_basic(const char *s);

int	lexer(t_shell *shell, const char *trimmed_line)
{
	t_lexer	*lex;

	lex = lexer_checks_basic(trimmed_line);
	if (lex->lexer != LEXER_SUCCESS)
	{
		// @todo handle errors
		// update_exit_status(shell, lex_error(lex->lexer));
		return (lex->lexer);
	}
	shell->token = tokenize(shell, trimmed_line);
	get_input(NULL);
	if (!shell->token)
		return (LEXER_NULL);
	if (!shell->token->cmd_args)
		return (destroy_all_tokens(shell), LEXER_NULL);
	return (LEXER_SUCCESS);
}
