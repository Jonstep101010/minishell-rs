#include "find_key.c"
#include "unity.h"
#include "arr_utils.c"
#include "occurs.c"

#include "print_arr_sep.c"

void	test_get_key_len() {
	char	*s = "KEY=val";
	TEST_ASSERT_EQUAL(3, get_key_len(s));
}
void	test_get_key_len_two() {
	char	*s = "V==val";
	TEST_ASSERT_EQUAL(1, get_key_len(s));
}

void	test_get_key_error() {
	char	*s = "KEYval";
	TEST_ASSERT_EQUAL(-1, get_key_len(s));
}

void	test_get_key_error_two() {
	char	*s = "=Vval";
	TEST_ASSERT_EQUAL(-1, get_key_len(s));
}

void	test_find_key() {
	TEST_ASSERT_EQUAL(3, get_key_len("bye=yes"));
	char	*arr[] = {"hello=wrong", "bye=yes", "night=wrong", NULL};
	int	index = find_key_env(arr, "bye=no", get_key_len);
	TEST_ASSERT_EQUAL(1, index);
}

void	test_find_key_error() {
	TEST_ASSERT_EQUAL(4, get_key_len("byes=wrong"));
	char	*arr[] = {"hello=wrong", "byes=wrong", "night=wrong", NULL};
	int	index = find_key_env(arr, "bye=no", get_key_len);
	TEST_ASSERT_EQUAL(-1, index);
}



