#include "unity.h"

#include "../src/append_array.c"
#include "../include/libft/src/arr/print_arr.c"
void	test_append() {
	char	*arr[] = {"Hello, ", "World", NULL};
	char	*expected[] = {"Hello, ", "World", "!", NULL};

	char	**tmp = append_2d_arr(arr, "!");
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tmp, 4);
	TEST_ASSERT_NULL(tmp[3]);
	print_arr((const char **)expected);
	print_arr((const char **)tmp);
	TEST_ASSERT_EQUAL(arr_len((const char **)arr) + 1, arr_len((const char **)expected));
	TEST_ASSERT_EQUAL(arr_len((const char **)tmp), arr_len((const char **)expected));
}
