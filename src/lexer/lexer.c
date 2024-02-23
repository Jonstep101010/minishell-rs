#include "libft.h"
#include "lexer.h"
#include "str_utils.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <unistd.h>
#include "minishell.h"

void	arr_free(char **arr);
t_lexer	lexer_checks_basic(char *s);
#ifndef TEST
// void	update_exit_status(t_shell *shell, int status);
// char	**split_outside_quotes(const char *to_split, char c);
void	add_pipe_split_as_tokens(char **pipe_split, t_shell *shell);
void	convert_split_token_string_array_to_tokens(t_shell *shell);
void	convert_tokens_to_string_array(t_token *token);
#include <sys/wait.h>

static int		lex_error(t_lexer code)
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

void	builtin_exit(t_shell *shell, t_token *token);

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

