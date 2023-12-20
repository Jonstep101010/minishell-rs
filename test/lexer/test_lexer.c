#include "../src/lexer.c"
#include "../src/while_string_wrapper.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>

/* @follow-up */
// void test_works_with_multiple_statements(void)

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
	TEST_ASSERT(lexer("\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n\n\n\n") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n\n\n\n\n\n\n\n\n\n") == LEXER_SUCCESS);
}

void test_works_with_brackets(void)
{
	TEST_ASSERT_EQUAL_INT8(LEXER_SUCCESS,
		lexer("echo \"This is a test\""));
	TEST_ASSERT_EQUAL_INT8(LEXER_SUCCESS,
		lexer("echo \"Hello, World!\""));
	TEST_ASSERT_EQUAL_INT8(2,
		lexer("echo 'Hello, World!"));
	TEST_ASSERT_EQUAL_INT8(LEXER_SUCCESS,
		lexer("awk '{count++} END {print count}'"));
}
// not sure why but bash also returns: syntax error near unexpected token
void test_returns_syntax_error_near_unexpected_token(void)
{
	TEST_ASSERT_EQUAL_INT8(LEXER_SUCCESS,
		lexer("if (x > 5) { printf(\"x is greater than 5\"); }"));
}

// void test_does_not_work_with_escapes(void)
// {
// 	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
// 		lexer("echo Hello, World!\""));
// 	// should return: !\': event not found
// 	// TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
// 	// 	lexer("echo Hello, World!\'"));
// 	// TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
// 	// 	lexer("echo \"Hello, World!\\\""));
// 	// TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
// 	// 	lexer("echo \'Hello, World!\\\'"));
// 	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
// 		lexer("echo \"Hello, World!"));
// 	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
// 		lexer("echo \'Hello, World!"));
// }


void	test_does_not_work_with_unbalanced_brackets(void)
{
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_BRACKETS, lexer("if (x > 5 { printf(\"x is greater than 5\"}); }"));
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_BRACKETS, lexer("while (i < 10) { i++; "));
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_BRACKETS, lexer("while (i < 10) i++;} "));
}

	// no need to handle semi-colons
	// TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_BRACKETS,
	// 	lexer("int x = 5; printf(The value of x is %d, x);"));
	// TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_BRACKETS,
	// 	lexer("for (int i = 0; i < 5; i++) { printf(\"%d\n\", i); }"));
	// should return: !\": event not found
	// TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_BRACKETS,
	// 	lexer("while (i < 10) { i++; }"));

