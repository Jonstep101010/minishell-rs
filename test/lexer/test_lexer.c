#include "../src/lexer/lexer.c"
#include "../src/lexer/lexer_support.c"
#include "../src/lexer/check_quotes.c"
#include "../src/lexer/check_pipes.c"
#include "../include/libft/src/string/str_cchr.c"
#include "../src/utils/while_string_wrapper.c"
#include "../src/utils/bool_array.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>

void test_redir_pipes_mix()
{
	TEST_ASSERT(lexer(">|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("<|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("> |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("< |") != LEXER_SUCCESS);
}

void test_works_with_newlines()
{
	TEST_ASSERT(lexer("\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n \n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n	\n \n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\\n\n\n\n\n") == LEXER_SUCCESS);
}

void test_works_with_quotes() {

	TEST_ASSERT(LEXER_SUCCESS == lexer("echo \"This is a test\""));
	TEST_ASSERT(LEXER_SUCCESS == lexer("echo \"Hello, World!\""));
}

void test_works_with_brackets(void)
{
	TEST_ASSERT(LEXER_SUCCESS == lexer("if (x > 5) { printf(\"x is greater than 5\"); }"));
	TEST_ASSERT(LEXER_SUCCESS == lexer("awk '{count++} END {print count}'"));
	TEST_ASSERT(LEXER_SUCCESS ==
		lexer("int x = 5; printf(The value of x is %d, x);"));
	TEST_ASSERT(LEXER_SUCCESS ==
		lexer("for (int i = 0; i < 5; i++) { printf(\"%d\n\", i); }"));
	TEST_ASSERT(LEXER_SUCCESS ==
		lexer("while (i < 10) { i++; }"));
}

//@audit-info no need to handle semi-colons
void	test_does_not_work_with_unbalanced_brackets(void)
{
	TEST_ASSERT(LEXER_SUCCESS != lexer("if (x > 5 { printf(\"x is greater than 5\"}); }"));
	TEST_ASSERT(LEXER_SUCCESS != lexer("while (i < 10) { i++; "));
	TEST_ASSERT(LEXER_SUCCESS != lexer("while (i < 10) i++;} "));
}

void	test_error() {
	TEST_ASSERT(LEXER_UNBALANCED_QUOTES
		== lexer("echo Hello, World!\""));
	TEST_ASSERT(LEXER_UNBALANCED_QUOTES
		== lexer("echo \"Hello, World!"));
	TEST_ASSERT(LEXER_UNBALANCED_QUOTES
		== lexer("echo \'Hello, World!"));
	// @audit-info no need to handle!
	TEST_ASSERT(LEXER_SUCCESS != lexer("echo 'Hello, World!"));
	TEST_ASSERT(LEXER_SUCCESS != lexer("'\"'"));
	TEST_ASSERT(LEXER_SUCCESS != lexer("echo \"'Hello, World!\""));
	TEST_ASSERT(LEXER_SUCCESS != lexer("\"'\""));
}
