#include "lexer.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <unistd.h>
#include "minishell.h"
#include "arr_utils.h"
#ifndef TEST

# include "tokens.h"
# include <sys/wait.h>

t_lexer	lexer_checks_basic(char *s);
void	builtin_exit(t_shell *shell, t_token *token);

static int	lex_error(t_lexer code)
{
	if (code == LEXER_SINGLE_QUOTE)
		return (eprint_single("syntax error near unexpected token '''\n"), 0);
	if (code == LEXER_DOUBLE_QUOTE)
		return (eprint_single("syntax error near unexpected token '\"'\n"), 0);
	if (code == LEXER_UNBALANCED_QUOTES)
		return (eprint_single("Error: quotes not closed\n"), 0);
	if (code == LEXER_PIPES)
		return (eprint_single("syntax error near unexpected token '|'\n"), 127);
	return (0);
}

t_lexer	lexer(t_shell *shell)
{
	t_lexer	code;

	code = lexer_checks_basic(shell->trimmed_line);
	if (code != LEXER_SUCCESS)
	{
		update_exit_status(shell, lex_error(code));
		return (code);
	}
	shell->split_pipes = split_outside_quotes(shell->trimmed_line, "|");
	if (!shell->split_pipes)
		builtin_exit(shell, NULL);
	add_pipe_split_as_tokens(shell->split_pipes, shell);
	if (!shell->token->split_pipes)
		return (arr_free(shell->split_pipes), LEXER_NULL);
	convert_split_token_string_array_to_tokens(shell);
	if (!shell->token->cmd_args)
		return (destroy_all_tokens(shell), LEXER_NULL);
	arr_free(shell->split_pipes);
	return (LEXER_SUCCESS);
}
#endif
