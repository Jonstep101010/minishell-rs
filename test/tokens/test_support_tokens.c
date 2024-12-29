#include "unity.h"
// this is a test for the support functions used in other tests

#include <stdlib.h>

#include "split_outside_quotes.c"
#include "utils.h"
#include "arr_utils.c"
#include "str_equal.c"
#include "expander.c"
#include "interpret_quotes.c"

// get module support
#include "support_tokens.c"


void	test_support_test_tokens(void)
{
	t_shell	*shell;
	shell = (t_shell *) support_test_tokens((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL});
	(void)shell;
	shell->token = get_tokens("ls -l $somedir ' ' | cat -e | wc -l");
	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' ' ", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING(" cat -e ", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING(" wc -l", shell->token[2].split_pipes);
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free(shell);
}

void test_support_test_tokens_cleanup(void)
{
	t_shell	*shell = support_test_tokens((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL});

	shell->token = get_tokens(" ls -l $somedir ' ' | cat -e | wc -l");

	TEST_ASSERT_EQUAL_STRING(" ls -l $somedir ' ' ", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING(" cat -e ", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING(" wc -l", shell->token[2].split_pipes);

	destroy_all_tokens(shell);
	tokenize(shell, " ls -l $somedir ' ' | cat -e | wc -l");
	TEST_ASSERT_NOT_NULL(shell->token->cmd_args);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING(" ", shell->token[0].cmd_args[3].elem);

	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	cleanup_support_test_token(shell);
}

void test_can_create_empty_env(void) {
	t_shell	*shell = support_clean_env((char *[]){"", NULL});
	TEST_ASSERT_EQUAL_STRING("", shell->env[0]);
	TEST_ASSERT_NULL(shell->env[1]);
	cleanup_support_test_token(shell);
}
