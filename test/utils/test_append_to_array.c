#include "unity.h"

#include "arr_utils.c"
#include "occurs.c"
#include "../include/libft/src/arr/print_arr.c"

void	test_append() {
	char	*tmp[] = {"Hello, ", "World", NULL};
	char	*expected[] = {"Hello, ", "World", "!", NULL};
	char	**arr = append_str_arr(tmp, "!");

	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
	TEST_ASSERT_NULL(arr[3]);
	TEST_ASSERT_EQUAL(arr_len((const char **)arr), arr_len((const char **)expected));
	TEST_ASSERT_EQUAL(arr_len((const char **)arr), arr_len((const char **)expected));
}

void	test_append_after_rm() {
	char	*tmp[] = {"World", NULL, NULL};
	char	**arr = append_str_arr(arr_dup((const char **)tmp), "Yapping");
	char	*expected[] = {"World", "Yapping", NULL};

	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 3);
	TEST_ASSERT_EQUAL(arr_len((const char **)expected), arr_len((const char **)arr));
	rm_str_arr(arr, "World");
	rm_str_arr(arr, "Yapping");
	char	*expected2[] = {NULL, NULL, NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected2, arr, 3);

	TEST_ASSERT_NOT_NULL(arr);
	arr = append_str_arr(arr, "Hello, ");
	TEST_ASSERT_NOT_NULL(arr[0]);
	arr = append_str_arr(arr, "World!");
	TEST_ASSERT_NOT_NULL(arr);

	char	*expected3[] = {"Hello, ", "World!", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected3, arr, 3);
}

void	test_create_null() {
	char	**arr = append_str_arr(NULL, "Hello");
	char	*expected[] = {"Hello", NULL};
	TEST_ASSERT_NOT_NULL(arr);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 2);
	arr = append_str_arr(arr, NULL);
	TEST_ASSERT_NULL(arr);
}
