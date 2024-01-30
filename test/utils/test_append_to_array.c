#include "libft.h"
#include "unity.h"

#include "arr_utils.c"
#include "occurs.c"
#include "../include/libft/src/arr/print_arr.c"
#include "utils.h"

#include "print_arr_sep.c"

// @follow-up fix test
// char	**append_once() {
// 	char	*tmp[] = {"Hello, ", "World", NULL};
// 	char	*expected[] = {"Hello, ", "World", "!", NULL};
// 	char	**arr = arr_dup((const char **)tmp);

// 	// print_arr(arr);
// 	TEST_ASSERT_NULL(arr[2]);
// 	arr = append_str_arr(arr, "!");

// 	// print_arr(arr);
// 	TEST_ASSERT_NULL(arr[3]);
// 	TEST_ASSERT_EQUAL_STRING("!", arr[2]);
// 	// char	**expected = arr_dup((const char **)tmp_expected);
// 	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);

// 	TEST_ASSERT_EQUAL(arr_len((const char **)arr), arr_len((const char **)expected));
// 	return (arr);
// }

// // fix test
// void	test_append() {
// 	char	**arr = append_once();
// 	print_arr(arr);
// 	char	*tmp_expected[] = {"Hello, ", "World", "!", "my", "precious", NULL};

// 	TEST_ASSERT_NULL(tmp_expected[5]);
// 	char	**expected = arr_dup((const char **)tmp_expected);
// 	arr = append_str_arr(arr, "my");
// 	TEST_ASSERT_NULL(arr[4]);
// 	print_arr(arr);

// 	arr = append_str_arr(arr, "precious");
// 	TEST_ASSERT_NULL(arr[5]);
// 	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 6);
// 	print_arr(arr);
// }

void	test_append_after_rm() {
	char	*tmp[] = {"World", NULL, NULL};
	char	**arr = append_str_arr(arr_dup((const char **)tmp), "Yapping");
	char	*expected[] = {"World", "Yapping", NULL, NULL};
	print_arr(arr);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
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
	TEST_ASSERT_EQUAL_STRING_ARRAY(arr_dup((const char **)expected3), arr, 3);
}

void	test_create_null() {
	char	**arr = append_str_arr(NULL, "Hello");
	char	*expected[] = {"Hello", NULL};
	TEST_ASSERT_NOT_NULL(arr);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 2);
	arr = append_str_arr(arr, NULL);
	TEST_ASSERT_NULL(arr);
}
