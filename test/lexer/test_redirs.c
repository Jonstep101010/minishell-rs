#include "support_lib.c"
#include "../src/lexer/checks_basic.c"
#include "../src/lexer/check_quotes.c"
#include "../src/lexer/check_pipes.c"
#include "../src/lexer/lexer_support.c"
#include "bool_array.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>
#include "error.c"

void test_handles_redirection(void)
{
	TEST_ASSERT(lexer_checks_basic("ls << infile >>") != LEXER_SUCCESS);
	// tests with ignored (inside quoted)

	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("ls '|' hello '>>'")->lexer);
}
void test_redir_smaller_valid()
{
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("ls < infile")->lexer);
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("ls << infile")->lexer);
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("< infile")->lexer);
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("<< infile")->lexer);
}

void test_redir_smaller_invalid()
{
	TEST_ASSERT(lexer_checks_basic("<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("<<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("<<<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("< < infile") != LEXER_SUCCESS);
}

void test_redir_greater_valid()
{
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("ls > outfile")->lexer);//truncates
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("ls >> outfile")->lexer);// appends
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic(">> outfile")->lexer);// creates outfile
}

void test_redir_greater_invalid()
{
	TEST_ASSERT(lexer_checks_basic("ls > outfile >") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("ls < infile <") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic(">") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer_checks_basic(">>") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer_checks_basic(">>>") != LEXER_SUCCESS);//syntax error
}
