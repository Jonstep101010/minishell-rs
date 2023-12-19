#include "../src/lexer.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>

/* @follow-up */
// void test_lexer_works_with_multiple_statements(void)

void test_lexer_pipes_redir(void)
{
	TEST_ASSERT(lexer(">|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("<|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("> |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("< |") != LEXER_SUCCESS);
}

/* @follow-up make sure these work */
void test_lexer_pipes_filter(void)
{
	TEST_ASSERT(lexer("|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("           |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("			|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("|   |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("| |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("|||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("||") == LEXER_SUCCESS);
}

void test_lexer_redir_smaller_valid(void)
{
	TEST_ASSERT(lexer("ls < infile") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("ls << infile") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("< infile") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("<< infile") == LEXER_SUCCESS);
}

void test_lexer_redir_smaller_invalid(void)
{
	TEST_ASSERT(lexer("<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("<<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("<<<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("< < infile") != LEXER_SUCCESS);
}

void test_lexer_redir_greater_valid(void)
{
	TEST_ASSERT(lexer("ls > outfile") == LEXER_SUCCESS);//truncates
	TEST_ASSERT(lexer("ls >> outfile") == LEXER_SUCCESS);// appends
	TEST_ASSERT(lexer(">> outfile") == LEXER_SUCCESS);// creates outfile
}

void test_lexer_redir_greater_invalid(void)
{
	TEST_ASSERT(lexer(">") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer(">>") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer(">>>") != LEXER_SUCCESS);//syntax error
}

void test_lexer_works_with_brackets(void)
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
void test_lexer_returns_syntax_error_near_unexpected_token(void)
{
	TEST_ASSERT_EQUAL_INT8(LEXER_SUCCESS,
		lexer("if (x > 5) { printf(\"x is greater than 5\"); }"));
}

void test_lexer_does_not_work_with_escapes(void)
{
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
		lexer("echo Hello, World!\""));
	// should return: !\': event not found
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
		lexer("echo Hello, World!\'"));
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
		lexer("echo \"Hello, World!\\\""));
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
		lexer("echo \'Hello, World!\\\'"));
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
		lexer("echo \"Hello, World!"));
	TEST_ASSERT_EQUAL_INT8(LEXER_UNBALANCED_QUOTES,
		lexer("echo \'Hello, World!"));
}

void	test_lexer_does_not_work_with_unbalanced_brackets(void)
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

