#include "unity.h"
#include "support_bool.h"
#include <stdlib.h>
#include <string.h>


#define START_INDEX 17
#define MAX_INDEX_DQ_ZERO 24

void test_range_ignore_both_0() {
	bool 	*arr = NULL;
	char	s[] = "this is my input \"'ignore'\"";
	bool_arr_zeroing(s, &arr, strlen(s));
	for (int i = 0; i <= (int)strlen(s); i++)
		TEST_ASSERT(false == arr[i]);
	range_ignore(s, &arr, '"');
	range_ignore(s, &arr, '\''); TEST_ASSERT(arr[24] == true);
	for (int i = 0; i < START_INDEX; i++)
		TEST_ASSERT_EQUAL(arr[i], false);
	for (int i = START_INDEX; i <= MAX_INDEX_DQ_ZERO; i++)
		TEST_ASSERT_EQUAL(arr[i], true);
	free(arr);
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
