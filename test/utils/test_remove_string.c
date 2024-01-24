#include "libft.h"
#include "unity.h"

#include "../src/utils/arr_utils.c"
#include "../include/libft/src/arr/print_arr.c"
#include "../src/utils/occurs.c"

void	test_arr_remove_hello() {
	char	*tmp[] = {"Hello, ", "World", "!", NULL};

	char	**arr = arr_dup((const char **)tmp);
	rm_str_arr(arr, "Hello, ");
	char	*expected[] = {"World", "!", NULL, NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
}

void	test_arr_remove() {
	char	*tmp[] = {"World", "!", NULL, NULL};
	char	*expected[] = {"World", NULL, NULL, NULL};
	char	**arr = arr_dup((const char **)tmp);

	rm_str_arr(arr, "!");
	TEST_ASSERT_EQUAL(arr_len((const char **)arr), arr_len((const char **)expected));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 3);
}
