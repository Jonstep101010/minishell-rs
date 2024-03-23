#include "checks_basic.c"
#include "lexer_support.c"
#include "check_quotes.c"
#include "check_pipes.c"
#include "while_string_wrapper.c"
#include "bool_array.c"
#include "unity.h"
#include <stdbool.h>
#include "support_lib.c"

void test_redir_pipes_mix()
{
	TEST_ASSERT(lexer_checks_basic(">|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("<|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("> |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("< |") != LEXER_SUCCESS);
}

void test_works_with_newlines()
{
	TEST_ASSERT(lexer_checks_basic("\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n \n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n	\n \n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\\n\n\n\n\n") == LEXER_SUCCESS);
}

void test_works_with_quotes() {

	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo \"This is a test\""));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo \"Hello, World!\""));
	TEST_ASSERT(LEXER_DOUBLE_QUOTE
		== lexer_checks_basic("echo Hello, World!\""));
	TEST_ASSERT(LEXER_DOUBLE_QUOTE
		== lexer_checks_basic("echo \"Hello, World!"));
	TEST_ASSERT(LEXER_SINGLE_QUOTE
		== lexer_checks_basic("echo \'Hello, World!"));
	TEST_ASSERT(LEXER_SUCCESS != lexer_checks_basic("echo 'Hello, World!"));
	TEST_ASSERT(LEXER_SUCCESS != lexer_checks_basic("'\"'"));
	TEST_ASSERT(LEXER_SUCCESS != lexer_checks_basic("echo \"'Hello, World!\""));
	TEST_ASSERT(LEXER_SUCCESS != lexer_checks_basic("\"'\""));
}

void test_trailing_leading_quote() {
	TEST_ASSERT(lexer_checks_basic("\"Hello, World!\"") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\"Hello, World!") == LEXER_DOUBLE_QUOTE);
	TEST_ASSERT(lexer_checks_basic("\"Hello, World!") == LEXER_DOUBLE_QUOTE);
	TEST_ASSERT(lexer_checks_basic("echo 'Hello, World!") == LEXER_SINGLE_QUOTE);
	TEST_ASSERT(lexer_checks_basic("echo Hello, World!'") == LEXER_SINGLE_QUOTE);
	TEST_ASSERT(lexer_checks_basic("echo \"Hello, World!'") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo Hello, World!\"") == LEXER_DOUBLE_QUOTE);
	TEST_ASSERT(lexer_checks_basic("echo \"Hello, World!\"") == LEXER_SUCCESS);
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("'echo'"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("if (x > 5) { printf(\"x is greater than 5\"); }"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("awk '{count++} END {print count}'"));
	TEST_ASSERT(LEXER_SUCCESS ==
		lexer_checks_basic("int x = 5; printf(The value of x is %d, x);"));
	TEST_ASSERT(LEXER_SUCCESS ==
		lexer_checks_basic("for (int i = 0; i < 5; i++) { printf(\"%d\n\", i); }"));
	TEST_ASSERT(LEXER_SUCCESS ==
		lexer_checks_basic("while (i < 10) { i++; }"));
}

// test () {} [] each and unbalanced aquivalents
void	test_parenthesis() {
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo (Hello, World!)"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo {Hello, World!}"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo Hello, World!}"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo Hello, World!)"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo (Hello, World!"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("if (x > 5 {{ printf(\"x is greater than 5\"})); }"));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("while (i < 10) { i++; "));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("while (i < 10) i++;} "));
	TEST_ASSERT(LEXER_SUCCESS == lexer_checks_basic("echo {Hello, World!"));
}
