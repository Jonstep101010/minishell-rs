#include "support_lib.c"
#include "../src/lexer/checks_basic.c"
#include "../src/lexer/check_quotes.c"
#include "../src/lexer/check_pipes.c"
#include "../src/lexer/lexer_support.c"
#include "../src/utils/bool_array.c"
#include "unity.h"
#include <stdbool.h>
#include "error.c"

/* @follow-up anything within balanced quotes */
void test_pipes_filter_WIP()
{
	TEST_ASSERT_EQUAL(lexer_checks_basic("echo \"|\"")->lexer, LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(lexer_checks_basic("echo '|'")->lexer, LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(lexer_checks_basic("\"|\"")->lexer, LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(lexer_checks_basic("'|'")->lexer, LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(lexer_checks_basic("echo \"Hello || World | hello\"")->lexer, LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(lexer_checks_basic("echo Hello | World | hello")->lexer, LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(lexer_checks_basic("echo Hello | World | hello")->lexer, LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(lexer_checks_basic("ls \"||\"| grep file '||'| wc -l")->lexer, LEXER_SUCCESS);
	// @audit-info make this an error (no handling)
	TEST_ASSERT(lexer_checks_basic("echo |")->lexer != LEXER_SUCCESS);
}

// @audit fix lexer to make these fail
void test_pipes_filter_fail()
{
	TEST_ASSERT(lexer_checks_basic("hello ||")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("hello |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT_NOT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("hello | hello | hello |")->lexer);
	TEST_ASSERT(lexer_checks_basic("     ||")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("           |    ")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("			|    ")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("        | |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("   <  |  < |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("   < infile |  < infiel| cat")->lexer == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("    |   |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("   < |||")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("      |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("||")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("||       ")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo hello || cat")->lexer == LEXER_SUCCESS);
}


// @audit fix lexer to make these fail
/* only needs to be input before opening a pipe */
void test_pipes_filter_working()
{
	TEST_ASSERT(lexer_checks_basic("|")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("           |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("			|")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("| |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("|   |")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("|||")->lexer != LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("      |")->lexer != LEXER_SUCCESS);
}

void test_pipes_filter_valid()
{
	TEST_ASSERT_EQUAL(lexer_checks_basic("hello | hello")->lexer, LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("hello|hello")->lexer == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("echo Hello | World")->lexer == LEXER_SUCCESS);
	TEST_ASSERT(lexer_checks_basic("ls | grep file | wc -l")->lexer == LEXER_SUCCESS);
	// sometimes works sometimes doesn't @audit
	TEST_ASSERT(lexer_checks_basic("hello || hello")->lexer == LEXER_SUCCESS);
	TEST_ASSERT_EQUAL(LEXER_SUCCESS, lexer_checks_basic("\n")->lexer);
}

