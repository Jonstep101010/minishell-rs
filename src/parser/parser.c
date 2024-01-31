#include "struct.h"
#include <stdlib.h>
#include "parser.h"
#include "utils.h"

// 1. handle expansion
// 2. handle quotes
char	**split_outside_quotes(const char *to_split, char c);
// @todo tests pipe splitting
// @todo redirs
int		parser(t_shell *shell)
{
	if (!shell->line)
		return (EXIT_FAILURE);
	shell->expanded_line = expand_variables(shell->line,
		(const char **)shell->owned_envp);
	if (!shell->expanded_line)
		return (EXIT_FAILURE);
	shell->tokens = split_outside_quotes(shell->expanded_line, '|');
	if (!shell->tokens)
		return (print_arr_sep(shell->tokens, '{', '}'), EXIT_FAILURE);
	// @audit give to childs everything after shell->tokens[0]
	// split for ourselves
	// shell->command = split_outside_quotes(
	// 	shell->tokens[0], ' ');
	// print_arr_sep(shell->command, '{', '}');
	// if (!interpret_quotes(shell->command))
	// 	return (EXIT_FAILURE);
	return (EXIT_SUCCESS);
}
