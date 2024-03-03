#include "support_lib.c"
#include "unity.h"

#include "../src/utils/arr_utils.c"
#include "../src/utils/str_equal.c"

#include "print_arr_sep.c"

void	test_arr_remove_hello() {
	char	*tmp[] = {"Hello, ", "World", "!", NULL};

	char	**arr = arr_dup((const char **)tmp);
	rm_str_arr(arr, "Hello, ");
	char	*expected[] = {"World", "!", NULL, NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
	arr_free(arr);
}

void	test_arr_remove() {
	char	*tmp[] = {"World", "!", NULL, NULL};
	char	*expected[] = {"World", NULL, NULL, NULL};
	char	**arr = arr_dup((const char **)tmp);

	rm_str_arr(arr, "!");
	TEST_ASSERT_EQUAL(arr_len((const char **)arr), arr_len((const char **)expected));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 3);
	arr_free(arr);
}
