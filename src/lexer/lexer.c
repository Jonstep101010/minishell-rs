#include "lexer.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <unistd.h>
#include "minishell.h"
#include "tokens.h"
#include <sys/wait.h>

t_lexer	lexer_checks_basic(const char *s);
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

char	*get_input(char *rl_prompt);

t_lexer	lexer(t_shell *shell, const char *trimmed_line)
{
	t_lexer	code;

	code = lexer_checks_basic(trimmed_line);
	if (code != LEXER_SUCCESS)
	{
		update_exit_status(shell, lex_error(code));
		return (code);
	}
	shell->token = tokenize(shell, trimmed_line);
	get_input(NULL);
	if (!shell->token)
		return (LEXER_NULL);
	if (!shell->token->cmd_args)
		return (destroy_all_tokens(shell), LEXER_NULL);
	return (LEXER_SUCCESS);
}
