#include "unity.h"
#include "support_lib.c"
#include "../src/lexer/lexer_support.c"

void test_count_number()
{
	struct s_lexer input;
	char s[] = "Hello 'world'! {This} is a [test] string.";

	count_number(s, &input);

	TEST_ASSERT_EQUAL(2, input.singlequotes);
	TEST_ASSERT_EQUAL(0, input.doublequotes);
	TEST_ASSERT_EQUAL(2, input.open_curly_brackets + input.close_curly_brackets);
	TEST_ASSERT_EQUAL(1, input.close_curly_brackets);
	TEST_ASSERT_EQUAL(2, input.open_square_brackets + input.close_square_brackets);
	TEST_ASSERT_EQUAL(1, input.close_square_brackets);
	TEST_ASSERT_EQUAL(0, input.open_parentheses);
	TEST_ASSERT_EQUAL(0, input.close_parentheses);
	char s2[] = "|< infile > outfile | \\";
	count_number(s2, &input);
	TEST_ASSERT_EQUAL(1, input.redir_greater);
	TEST_ASSERT_EQUAL(1, input.redir_smaller);
	TEST_ASSERT_EQUAL(2, input.pipes);
	char s3[] = "ls | grep\" file\" |'|' wc -l";
	count_number(s3, &input);
	TEST_ASSERT_EQUAL(3, input.pipes);
	TEST_ASSERT_EQUAL(2, input.doublequotes);
	TEST_ASSERT_EQUAL(2, input.singlequotes);
}
