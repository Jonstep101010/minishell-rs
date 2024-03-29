#include "lexer.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <stdlib.h>
#include <unistd.h>
#include "minishell.h"
#include "tokens.h"
#include <sys/wait.h>

t_lexer	*lexer_checks_basic(const char *s);

int	lexer(t_shell *shell, const char *trimmed_line)
{
	t_lexer	*lex;

	lex = lexer_checks_basic(trimmed_line);
	if (!lex->result)
	{
		// @todo handle errors
		update_exit_status(shell, lex->lexer);
		free(lex);
		return (1);
	}
	free(lex);
	shell->token = tokenize(shell, trimmed_line);
	get_input(NULL);
	if (!shell->token)
		return (-1);
	if (!shell->token->cmd_args)
		return (destroy_all_tokens(shell), -1);
	return (LEXER_SUCCESS);
}
