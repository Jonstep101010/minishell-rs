#ifndef SUPPORT_SHELL_C
# define SUPPORT_SHELL_C

#include "unity.h"
#include "struct.h"
#include <stdlib.h>
#include <string.h>
#include "support_lib.c"

t_shell	*setup_shell(char *line) {
	t_shell *shell = (t_shell *) calloc(sizeof(t_shell), 1);
	if (line)
		shell->line = strdup(line);
	if (!shell)
		TEST_FAIL_MESSAGE("Failed to allocate memory for shell");
	return (shell);
}

void	clean_shell(t_shell *shell) {
	free(shell);
}

void	debug_print_arr(t_shell *this, t_token *token)
{
	if (!this || !token || !token->command)
		return ;
	for (int i = 0; token->command[i]; i++)
	{
		fprintf(stderr, "string in arr:%s\n", token->command[i]);
	}
}
#endif
