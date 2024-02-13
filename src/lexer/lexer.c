#include "libft.h"
#include "lexer.h"
#include "struct.h"
#include "utils.h"
#include <stdbool.h>
#include <unistd.h>

t_lexer	lexer_checks_basic(char *s);
#ifndef TEST
// char	**split_outside_quotes(const char *to_split, char c);
void	add_pipe_split_as_tokens(char **pipe_split, t_shell *shell);
void	convert_split_token_string_array_to_tokens(t_shell *shell);
void	convert_tokens_to_string_array(t_token *token);
#include <sys/wait.h>

t_token	*lexer(t_shell *shell)
{
	if (lexer_checks_basic(shell->line) != LEXER_SUCCESS)
	{
		// printf("syntax error\n");
		shell->exit_status = 2;
		return (NULL);
	}
	// build tokens
	shell->split_pipes = split_outside_quotes(shell->line, "|");
	if (!shell->split_pipes)
		return (NULL);
	add_pipe_split_as_tokens(shell->split_pipes, shell);
	if (!shell->token->split_pipes)
		return (NULL);
	convert_split_token_string_array_to_tokens(shell);
	if (!shell->token->cmd_args)
		return (NULL);
	arr_free(shell->split_pipes);
	return (shell->token);
}
#endif

