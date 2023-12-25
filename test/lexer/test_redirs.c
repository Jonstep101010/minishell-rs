#include "../src/lexer/lexer.c"
#include "../src/lexer/check_quotes.c"
#include "../src/lexer/check_pipes.c"
#include "../src/lexer/lexer_support.c"
#include "../src/while_string_wrapper.c"
#include "../src/bool_array.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>

void test_handles_redirection(void)
{
	TEST_ASSERT(lexer("ls << infile >>") != LEXER_SUCCESS);
}
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
	TEST_ASSERT(lexer("ls > outfile >") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("ls < infile <") != LEXER_SUCCESS);
	TEST_ASSERT(lexer(">") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer(">>") != LEXER_SUCCESS);//syntax error
	TEST_ASSERT(lexer(">>>") != LEXER_SUCCESS);//syntax error
}
