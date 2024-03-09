#include "unity.h"
// this is a test for the support functions used in other tests

#include <stdlib.h>

#include "split_outside_quotes.c"
#include "utils.h"
#include "arr_utils.c"
#include "str_equal.c"
#include "token_utils.c"
#include "support_commands.c"
#include "destroy_tokens.c"
#include "error.c"
#include "build_command.c"
// environment
#include "build_tokens.c"
#include "expander.c"
#include "expand_variables.c"
#include "expand_var.c"
#include "interpret_quotes.c"

// get module support
#include "support_tokens.c"
void	test_support_test_tokens(void)
{
	t_shell	*shell;
	shell = (t_shell *) support_test_tokens("ls -l $somedir ' ' | cat -e | wc -l", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL});
	(void)shell;
	add_pipes_as_tokens(shell);
	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' '", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);
	free(shell->token);
	arr_free(shell->split_pipes);
	arr_free(shell->env);
	free(shell->trimmed_line);
	free(shell);
}

void test_support_test_tokens_cleanup(void)
{
	t_shell	*shell = support_test_tokens(" ls -l $somedir ' ' | cat -e | wc -l", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL});

	add_pipes_as_tokens(shell);

	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' '", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);

	convert_split_token_string_array_to_tokens(shell);
	TEST_ASSERT_NOT_NULL(shell->token->cmd_args);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING(" ", shell->token[0].cmd_args[3].elem);

	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_INT(SINGLE, shell->token[0].cmd_args[3].quote);
	TEST_ASSERT_EQUAL_INT(STRING, shell->token[0].cmd_args[2].type);
	TEST_ASSERT_EQUAL_INT(NONE, shell->token[0].cmd_args[2].quote);

	cleanup_support_test_token(shell);
}
