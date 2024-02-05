#include "../include/libft/src/string/str_cchr.c"
#include "../src/lexer/lexer.c"
#include "../src/lexer/check_quotes.c"
#include "../src/lexer/check_pipes.c"
#include "../src/lexer/lexer_support.c"
#include "../src/utils/while_string_wrapper.c"
#include "../src/utils/bool_array.c"
#include "unity.h"
#include <stdbool.h>

/* @follow-up anything within balanced quotes */
void test_pipes_filter_WIP()
{
	TEST_ASSERT(lexer_checks_basic("echo \"|\"") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo '|'") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\"|\"") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("'|'") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo \"Hello || World | hello\"") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo Hello | World | hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo Hello | World | hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("ls \"||\" grep file '||' wc -l") == LEXER_SUCCESS);
	// @audit-info make this an error (no handling)
	// TEST_ASSERT(lexer_checks_basic("echo |") == LEXER_SUCCESS);
}

// @audit fix lexer to make these fail
void test_pipes_filter_fail()
{
	TEST_ASSERT(lexer_checks_basic("hello ||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("hello |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("     ||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("           |    ") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("			|    ") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("        | |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("   <  |  < |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("    |   |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("   < |||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("      |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("||       ") != LEXER_SUCCESS);
}


// @audit fix lexer to make these fail
/* only needs to be input before opening a pipe */
void test_pipes_filter_working()
{
	TEST_ASSERT(lexer_checks_basic("|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("           |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("			|") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("| |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("|   |") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("|||") != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("      |") != LEXER_SUCCESS);
}

void test_pipes_filter_valid()
{
	TEST_ASSERT(lexer_checks_basic("hello | hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("hello|hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo Hello | World") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("ls | grep file | wc -l") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("hello || hello") == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("\n") == LEXER_SUCCESS);
}

