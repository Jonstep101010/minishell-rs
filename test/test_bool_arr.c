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
	while (i < 5)
	{
		TEST_ASSERT_EQUAL(false, arr[i]);
		i++;
	}
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

static void support_ranges_test0(bool *arr) {
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

void support_print(const char *s, bool *arr) {
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

void test_ignore_can_work0()
{
	bool 	*arr = NULL;
	char	s[] = "this is my input \"ignore\" 't' 'this' \"is\" 'a' \"test\" 'string'";
	int len = (int)strlen(s);

	bool_arr_zeroing(s, &arr, len);
	range_ignore(s, &arr, '\"');
	range_ignore(s, &arr, '\'');
	support_ranges_test0(arr);
	TEST_ASSERT_EQUAL('\0', s[61]);
	free(arr);
}
