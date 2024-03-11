#include "unity.h"
#include "tokens.h"
#include "struct.h"
#include "init.c"
#include "support_lib.c"
#include "destroy_tokens.c"
#include "support_commands.c"
#include "build_command.c"
#include "token_utils.c"
#include "build_tokens.c"
#include <stdlib.h>

t_shell	*support_test_tokens(char const *line, char *envp[])
{
	t_shell	*shell;
	shell = init_shell(envp);
	shell->exit_status = 0;
	shell->line = ft_strdup(line);
	shell->trimmed_line = ft_strtrim(shell->line, WHITESPACE);
	TEST_ASSERT_NOT_NULL(shell->trimmed_line);
	free(shell->line);
	return (shell);
}

void	cleanup_support_test_token(t_shell *shell)
{
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free_null(&shell->trimmed_line);
	arr_free(shell->split_pipes);
	free_null(&shell);
}
