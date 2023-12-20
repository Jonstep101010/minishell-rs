#include "../src/lexer.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>

void test_redir_smaller_valid()
{
	TEST_ASSERT(lexer("ls < infile") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("ls << infile") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("< infile") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("<< infile") == LEXER_SUCCESS);
}

void test_redir_smaller_invalid()
{
	TEST_ASSERT(lexer("<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("<<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("<<<") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("< < infile") != LEXER_SUCCESS);
}

void test_redir_greater_valid()
{
	TEST_ASSERT(lexer("ls > outfile") == LEXER_SUCCESS);//truncates
	TEST_ASSERT(lexer("ls >> outfile") == LEXER_SUCCESS);// appends
	TEST_ASSERT(lexer(">> outfile") == LEXER_SUCCESS);// creates outfile
}

void test_redir_greater_invalid()
{
	TEST_ASSERT(lexer(">") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer(">>") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer(">>>") != LEXER_SUCCESS);//syntax error
}
