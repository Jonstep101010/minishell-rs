#include "struct.h"
#include "libft.h"
#include <stdlib.h>
#include "parser.h"


// 1. handle expansion
// 2. handle quotes

// @todo pipes
// @todo redirs
int		parser(t_shell *shell)
{
	if (!shell->line)
		return (EXIT_FAILURE);
	split_command(shell);
	if (!shell->command)
		return (EXIT_FAILURE);
	// @audit do expansion -> replace $KEY with value
	expand_variables(shell);
	// expansion should be done by this point
	if (!interpret_quotes(shell->command))
		return (EXIT_FAILURE);
	return (EXIT_SUCCESS);
}
