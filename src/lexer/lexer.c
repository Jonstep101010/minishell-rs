#include "lexer.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <unistd.h>
#include "minishell.h"
#include "tokens.h"
#include <sys/wait.h>


// int	lex_error(t_lexer code)
// {
// 	if (code == LEXER_SINGLE_QUOTE)
// 		return (eprint_single("syntax error near unexpected token '''\n"), 0);
// 	if (code == LEXER_DOUBLE_QUOTE)
// 		return (eprint_single("syntax error near unexpected token '\"'\n"), 0);
// 	if (code == LEXER_UNBALANCED_QUOTES)
// 		return (eprint_single("Error: quotes not closed\n"), 0);
// 	if (code == LEXER_PIPES)
// 		return (eprint_single("syntax error near unexpected token '|'\n"), 127);
// 	return (0);
// }

t_lexer	*lexer_checks_basic(const char *s);
char	*get_input(char *rl_prompt);

int	lexer(t_shell *shell, const char *trimmed_line)
{
	t_lexer	*lex;

	lex = lexer_checks_basic(trimmed_line);
	if (lex->lexer != LEXER_SUCCESS)
	{
		// update_exit_status(shell, lex_error(lex->lexer));
		return (lex->lexer);
	}
	// if (code != LEXER_SUCCESS)
	// 	eprint("would not pass: %d", code);
	// if (code != 0)
	// {
	// 	update_exit_status(shell, lex_error(code));
	// 	return (code);
	// }
	shell->token = tokenize(shell, trimmed_line);
	get_input(NULL);
	if (!shell->token)
		return (LEXER_NULL);
	if (!shell->token->cmd_args)
		return (destroy_all_tokens(shell), LEXER_NULL);
	return (LEXER_SUCCESS);
}
