#include "../src/lexer.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>

// void test_pipes_filter_WIP()
// {
// }

void test_pipes_filter_fail()
{
	TEST_ASSERT(lexer("hello ||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("hello |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("     ||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("           |    ") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("			|    ") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("        | |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("   <  |  < |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("    |   |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("   < |||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("      |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("||       ") != LEXER_SUCCESS);
}

/* handle this: "cmd ||" (open cmd (wait for second:
	 (does not have to be valid))) ? -> only for bonus*/
/* handle this: "cmd |" (neither cmd has to be valid) ? -> only for bonus*/
/* only needs to be input before opening a pipe */
/* @follow-up make sure these work */
void test_pipes_filter_working()
{
	TEST_ASSERT(lexer("|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("           |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("			|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("| |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("|   |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("|||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer("      |") != LEXER_SUCCESS);
}

void test_pipes_filter_valid()
{
	TEST_ASSERT(lexer("hello | hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("hello || hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n") == LEXER_SUCCESS);
}

