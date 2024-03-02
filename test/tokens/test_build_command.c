#include "unity.h"
// #include "struct.h"
// #include "tokens.h"
// #include "init.c"
// #include "support_lib.c"
// #include "destroy_tokens.c"
// #include "arr_utils.c"
// #include "support_commands.c"
// #include "utils.h"
// #include "str_equal.c"
// #include "error.c"
// #include "free_strjoin.c"

#include "tokens.h"
#include "struct.h"
#include <stdlib.h>

#include "split_outside_quotes.c"
#include "support_lib.c"
#include "utils.h"
#include "arr_utils.c"
#include "str_equal.c"
#include "token_utils.c"
#include "support_commands.c"
#include "destroy_tokens.c"
#include "error.c"
#include "build_command.c"
#include "init.c"


#include "build_tokens.c"
#include "expander.c"
#include "expand_variables.c"
#include "interpret_quotes.c"

void	test_convert_string_array_to_tokens(void)
{
	t_shell	*shell;

	// input like "ls -l somedir | cat -e | wc -l";
	shell = init_shell((const char **)((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL}));
	shell->line = ft_strdup(" ls -l $somedir ' ' | cat -e | wc -l");
	shell->trimmed_line = ft_strtrim(shell->line, WHITESPACE);
	free(shell->line);
	TEST_ASSERT_NOT_NULL(shell->trimmed_line);

	// add the split tokens as tokens
	add_pipes_as_tokens(shell);

	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' '", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);

	destroy_all_tokens(shell);
	arr_free(shell->owned_envp);
	free_null(&shell->trimmed_line);
	arr_free(shell->split_pipes);
	free_null(&shell);
}
