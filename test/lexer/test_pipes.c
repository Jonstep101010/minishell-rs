#include "../include/libft/src/string/str_cchr.c"
#include "../src/lexer/lexer.c"
#include "../src/lexer/check_quotes.c"
#include "../src/lexer/check_pipes.c"
#include "../src/lexer/lexer_support.c"
#include "../src/utils/while_string_wrapper.c"
#include "../src/utils/bool_array.c"
#include "struct.h"
#include "unity.h"
#include <stdbool.h>

/* @follow-up anything within balanced quotes */
void test_pipes_filter_WIP()
{
	TEST_ASSERT(lexer("echo \"|\"") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("echo '|'") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\"|\"") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("'|'") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("echo \"Hello || World | hello\"") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("echo Hello | World | hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("echo Hello | World | hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("ls \"||\" grep file '||' wc -l") == LEXER_SUCCESS);
	// @audit-info make this an error (no handling)
	// TEST_ASSERT(lexer("echo |") == LEXER_SUCCESS);
}

// @audit fix lexer to make these fail
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


// @audit fix lexer to make these fail
/* only needs to be input before opening a pipe */
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
	TEST_ASSERT(lexer("hello|hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("echo Hello | World") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("ls | grep file | wc -l") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("hello || hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer("\n") == LEXER_SUCCESS);
}

