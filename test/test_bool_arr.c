#include "libft.h"
#include "struct.h"
#include "minishell.h"
#include "../src/while_string_wrapper.c"
#include "../src/lexer.c"
#include "../src/bool_array.c"
#include "unity.h"
#include <stdbool.h>
#include <string.h>

void test_bool_arr_can_have_a_true_value() {
	bool	*arr = NULL;
	int		i	 = 0;

	bool_arr_fixture("Hello", &arr, 5);
	for (i = 0; i < (int)strlen("Hello");i++)
		TEST_ASSERT_EQUAL(false, arr[i]);
	TEST_ASSERT_EQUAL(true, arr[i]);
	free(arr);
}

void test_range_ignore_sq_0() {
	bool 	*arr = NULL;
	char	s[] = "this is my input 'ignore'";
	bool_arr_zeroing(s, &arr, strlen(s));
	for (int i = 0; i <= (int)strlen(s); i++)
		TEST_ASSERT(false == arr[i]);
	range_ignore(s, &arr, '\'');
	TEST_ASSERT(arr[24] == true);
	for (int i = 0; i <= 16; i++)
		TEST_ASSERT_EQUAL(arr[i], false);
	for (int i = 17; i <= 24; i++)
		TEST_ASSERT_EQUAL(arr[i], true);
	free(arr);
}

void test_range_ignore_dq_0() {
	bool 	*arr = NULL;
	char	s[] = "this is my input \"ignore\"";
	bool_arr_zeroing(s, &arr, strlen(s));
	for (int i = 0; i <= (int)strlen(s); i++)
		TEST_ASSERT(false == arr[i]);
	range_ignore(s, &arr, '"'); TEST_ASSERT(arr[24] == true);
	for (int i = 0; i <= 16; i++)
		TEST_ASSERT_EQUAL(arr[i], false);
	for (int i = 17; i <= 24; i++)
		TEST_ASSERT_EQUAL(arr[i], true);
	free(arr);
}

void test_range_ignore_both_0() {
	bool 	*arr = NULL;
	char	s[] = "this is my input \"'ignore'\"";
	bool_arr_zeroing(s, &arr, strlen(s));
	for (int i = 0; i <= (int)strlen(s); i++)
		TEST_ASSERT(false == arr[i]);
	range_ignore(s, &arr, '"');
	range_ignore(s, &arr, '\''); TEST_ASSERT(arr[24] == true);
	for (int i = 0; i <= 16; i++)
		TEST_ASSERT_EQUAL(arr[i], false);
	for (int i = 17; i <= 24; i++)
		TEST_ASSERT_EQUAL(arr[i], true);
	free(arr);
}

static void support_ranges_test0(const bool *arr) {
	for (int i = 0; i <= 16; i++)
		TEST_ASSERT_FALSE(arr[i]);
	for (int i = 17; i <= 24; i++)
		TEST_ASSERT_TRUE(arr[i]);
	TEST_ASSERT_FALSE_MESSAGE(arr[25], "s[25] is a space and not in quotes");
	for (int i = 26; i <= 28; i++)
		TEST_ASSERT_TRUE(arr[i]);
	TEST_ASSERT_FALSE_MESSAGE(arr[29], "s[29] is a space and not in quotes");
	for (int i = 30; i <= 35; i++)
		TEST_ASSERT_TRUE(arr[i]);
	TEST_ASSERT_FALSE_MESSAGE(arr[36], "s[36] is a space and not in quotes");
	for (int i = 37; i <= 40; i++)
		TEST_ASSERT_TRUE(arr[i]);
	TEST_ASSERT_FALSE_MESSAGE(arr[41], "s[41] is a space and not in quotes");
	for (int i = 42; i <= 44; i++)
		TEST_ASSERT_TRUE(arr[i]);
	TEST_ASSERT_FALSE_MESSAGE(arr[45], "s[45] is a space and not in quotes");
	for (int i = 46; i <= 51; i++)
		TEST_ASSERT_TRUE(arr[i]);
	TEST_ASSERT_FALSE_MESSAGE(arr[52], "s[52] is a space and not in quotes");
	for (int i = 53; i <= 60; i++)
	TEST_ASSERT_TRUE(arr[60]);
	TEST_ASSERT_FALSE(arr[61]);
}

void support_print(const char *s, const bool *arr) {
	for (int i = 0; i <= (int)strlen(s); i++){
		fprintf(stderr, "i: %d, arr[i]: %d, s[i]: %c\n", i, arr[i], s[i]);
	}
	for (int i = 0; i <= (int)strlen(s); i++){
		if (arr[i] == true)
			fprintf(stderr, "%d", arr[i]);
		else
			fprintf(stderr, "%d", arr[i]);
	}
	fprintf(stderr,"\n");
}

bool *support_expected(const char *s) {
	// convert char array to bool array

	bool 	*arr = NULL;
	int		len = (int)strlen(s);

	bool_arr_zeroing(s, &arr, len);
	if (!arr)
		return (NULL);
	for (int i = 0; i <= len; i++) {
		if (s[i] == '1')
			arr[i] = true;
		else
			arr[i] = false;
	}
	return (arr);
}

void test_support_expected() {
	char	s[] = "0110011100011111000";
	// (_""he'y'ayo"the"lay)
	bool	expected[20] = {false, true, true, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, false, false};
	bool	*actual = support_expected(s);
	if (!actual)
		TEST_FAIL();
	for (int i = 0; i <= (int)strlen(s); i++)
	{
		fprintf(stderr, "i: %d, actual[i]: %d, expected[i]: %d\n", i, actual[i], expected[i]);
		TEST_ASSERT_EQUAL(expected[i], actual[i]);
	}
	free(actual);
}

void test_support_expected2() {
	char s[] = "101010101010101110";
	bool expected[20] = {true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, false, false};
	bool *actual = support_expected(s);
	if (!actual)
		TEST_FAIL();
	for (int i = 0; i <= (int)strlen(s); i++){
		fprintf(stderr, "i: %d, actual[i]: %d, expected[i]: %d\n", i, actual[i], expected[i]);
		TEST_ASSERT_EQUAL(expected[i], actual[i]);
	}
	free(actual);
}

void test_ignore_can_work0()
{
	bool 	*arr = NULL;
	char	s[] = "this is my input \"ignore\" 't' 'this' \"is\" 'a' \"test\" 'string'";
	bool	*expected = support_expected("00000000000000000111111110111011111101111011101111110111111110");
	int len = (int)strlen(s);

	bool_arr_zeroing(s, &arr, len);
	range_ignore(s, &arr, '\"');
	range_ignore(s, &arr, '\'');
	support_print(s, arr);
	for (int i = 0; i < (int)strlen(s); i++) {
		TEST_ASSERT_EQUAL(expected[i], arr[i]);
	}
	support_ranges_test0(arr);
	TEST_ASSERT_EQUAL('\0', s[61]);
	free(arr);
}

struct s_test {
	bool	*expected;
	bool	*actual;
	int		slen;
};

/* boilerplate to prep tests */
struct s_test *returns_test_struct(const char *s, const char *expected) {
	struct s_test *test = calloc(1, sizeof(struct s_test));
	if (!test)
		return (NULL);
	test->actual = NULL;
	test->slen = (int)strlen(s);

	bool_arr_zeroing(s, &(test->actual), test->slen);
	if (!test->actual)
		return (NULL);
	range_ignore(s, &(test->actual), '\'');
	range_ignore(s, &(test->actual), '\"');
	test->expected = support_expected(expected);
	return (test);
}

void cleanup_test_struct(struct s_test *test) {
	free(test->expected);
	free(test->actual);
	free(test);
}

void test_ignore_can_work1()
{
	bool 	*actual = NULL;
	char	s[] = "\"'\"";
	int len = (int)strlen(s);

	bool_arr_zeroing(s, &actual, len);
	range_ignore(s, &actual, '\'');
	range_ignore(s, &actual, '\"');

	bool *expected = support_expected("1110");
	for (int i = 0; i <= len; i++) {
		TEST_ASSERT_EQUAL(expected[i], actual[i]);
	}
}

		// fprintf(stderr, "i: %d, actual[i]: %d, expected[i]: %d\n", i, test->actual[i], test->expected[i]);
void test_can_work_test_struct()
{
	struct s_test *test = returns_test_struct("\"'\"", "1110");
	for (int i = 0; i <= test->slen; i++) {
		TEST_ASSERT_EQUAL(test->expected[i], test->actual[i]);
	}
	cleanup_test_struct(test);
}

void test_can_work_2()
{
	struct s_test *test = returns_test_struct("this is my input \"ignore\" 't' 'this' \"is\" 'a' \"test\" 'string'", "00000000000000000111111110111011111101111011101111110111111110");
	for (int i = 0; i <= test->slen; i++) {
		TEST_ASSERT_EQUAL(test->expected[i], test->actual[i]);
	}
	support_ranges_test0(test->actual);
	cleanup_test_struct(test);
}
