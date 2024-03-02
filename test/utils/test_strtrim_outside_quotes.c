#include "unity.h"
#include "strtrim_outside_quotes.c"
#include "support_lib.c"

void	test_can_respect_quotes() {
	char	*input = "  '  '  ";
	char	*expected = "'  '";
	char	*actual = strtrim_outside_quotes(input, " ");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void test_can_trim_outside_quotes() {
	char *input = "   hello world   ";
	char *expected = "hello world";
	char *actual = strtrim_outside_quotes(input, " ");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void test_can_trim_outside_quotes_with_multiple_sets() {
	char *input = "   hello world   ";
	char *expected = "hello wor";
	char *actual = strtrim_outside_quotes(input, " lode");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void test_can_handle_empty_string() {
	char *input = "";
	char *expected = "";
	char *actual = strtrim_outside_quotes(input, " ");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void test_can_handle_null_string() {
	char *input = NULL;
	char *expected = NULL;
	char *actual = strtrim_outside_quotes(input, " ");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void test_can_handle_null_set() {
	char *input = "   hello world   ";
	char *expected = "   hello world   ";
	char *actual = strtrim_outside_quotes(input, NULL);
	TEST_ASSERT_EQUAL_STRING(expected, actual);
}

void	test_with_leading_trailing() {
	char	*input = "||echo $somedir|";
	char	*expected = "echo $somedir";
	char	*actual = strtrim_outside_quotes(input, "|");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void	test_with_front_trim() {
	char	*input = "||echo $somedir|";
	char	*expected = "echo $somedir|";
	char	*actual = strtrim_front_outside_quotes(input, "|");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void	test_with_rear_trim_after() {
	char	*input = "echo $somedir|";
	char	*expected = "echo $somedir";
	char	*actual = strtrim_back_outside_quotes(input, "|");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void	test_with_rear_trim() {
	char	*input = "||echo $somedir|";
	char	*expected = "||echo $somedir";
	char	*actual = strtrim_back_outside_quotes(input, "|");
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

# ifndef WHITESPACE
#  define WHITESPACE " \t\n\r\v\f"
# endif

void	test_with_leading_trailing_whitespace() {
	char	*input = "         echo $somedir' '           ";
	char	*expected = "echo $somedir' '";
	char	*actual = strtrim_outside_quotes(input, WHITESPACE);
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void	test_front() {
	char	*input = "         echo $somedir' '           ";
	char	*expected = "echo $somedir' '           ";
	char	*actual = strtrim_front_outside_quotes(input, WHITESPACE);
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void	test_rear() {
	char	*input = "         echo $somedir' '           ";
	char	*expected = "         echo $somedir' '";
	char	*actual = strtrim_back_outside_quotes(input, WHITESPACE);
	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}

void	test_leading_trailing() {
	char	*input = "||echo $somedir|";
	char	*expected = "echo $somedir";
	char	*actual = strtrim_outside_quotes(input, "|");

	TEST_ASSERT_EQUAL_STRING(expected, actual);
	free(actual);
}
