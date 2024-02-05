#include "unity.h"
#include "libft.h"
// TEST_INCLUDE_PATH("../include/libft/src/")

void	test_arr_len() {
	char	*arr[] = {"0", "1", "2", "3", "4", NULL};
	TEST_ASSERT_NOT_NULL(arr);
	TEST_ASSERT_EQUAL(5, arr_len((const char **)arr));
}

void	test_arr_dup() {
	char	*arr[] = {"0", "1", "2", "3", "4", NULL};
	TEST_ASSERT_NOT_NULL(arr);
	char	**copy = arr_dup((const char **)arr);
	TEST_ASSERT_NOT_NULL(copy);
	TEST_ASSERT_EQUAL(5, arr_len((const char **)copy));
	TEST_ASSERT_EQUAL_STRING_ARRAY(arr, copy, 6);
	arr_free(copy);
}

void	test_arr_null() {
	char	**arr = NULL;
	TEST_ASSERT_NULL(arr);
	TEST_ASSERT_EQUAL(0, arr_len((const char **)arr));
	char	**actual = arr_dup((const char **)arr);
	TEST_ASSERT_NULL(actual);
	arr_free(actual);
}

void	test_arr_free() {
	char	**arr = NULL;
	TEST_ASSERT_NULL(arr);
	char	**actual = (char **)calloc(1, sizeof(char *));
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_NULL(actual[0]);
	char	**copy = arr_dup((const char **)actual);
	TEST_ASSERT_NOT_NULL(copy);
	TEST_ASSERT_NULL(copy[0]);
	arr_free(actual);
	arr_free(copy);
	TEST_ASSERT_NULL(arr_dup((const char **)NULL));
}

#include "print_arr_sep.c"
#include "occurs.c"
#include "arr_utils.c"

void	test_arr_trim_one(void)
{
	char	**arr = (char *[]){"  ls -l somedir ", " cat -e ", " wc -l", NULL};
	char	**expected = (char *[]){"ls -l somedir", "cat -e", "wc -l", NULL};
	char	**ret = arr_trim(arr, " ");
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, ret, 4);
	arr_free(ret);
}

void	test_arr_trim_two(void)
{
	char	**arr = (char *[]){"  ls -l somedir ", " cat -e ", " wc -l", NULL};
	char	**expected = (char *[]){"ls -l somedir", "cat -e", "wc -l", NULL};
	char	**ret = arr_trim(arr, " ");
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, ret, 4);
	arr_free(ret);
}
