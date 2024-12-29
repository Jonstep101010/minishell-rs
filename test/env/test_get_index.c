#include "get_index.c"
#include "unity.h"
#include "arr_utils.c"
#include "str_equal.c"

#include "support_lib.c"

void	test_get_key_len() {
	char	*s = "KEY=val";
	TEST_ASSERT_EQUAL(3, get_key_len(s));
}
void	test_get_key_len_two() {
	char	*s = "V==val";
	TEST_ASSERT_EQUAL(1, get_key_len(s));
}

void	test_get_key_error_two() {
	char	*s = "=Vval";
	TEST_ASSERT_EQUAL(-1, get_key_len(s));
}

void	test_find_key() {
	TEST_ASSERT_EQUAL(3, get_key_len("bye=yes"));
	char	*arr[] = {"hello=wrong", "bye=yes", "night=wrong", NULL};
	int	index = get_index_env(arr, "bye=no");
	TEST_ASSERT_EQUAL(1, index);
}

void	test_find_key_error() {
	TEST_ASSERT_EQUAL(4, get_key_len("byes=wrong"));
	char	*arr[] = {"hello=wrong", "byes=wrong", "night=wrong", NULL};
	int	index = get_index_env(arr, "bye=no");
	TEST_ASSERT_EQUAL(-1, index);
}

void	test_find_key_one() {
	// get_index_env(char *const *env, const char *s)

	char	*env[] = {"hello=world", "bye=yes", "night=wrong", NULL};

	int	index = get_index_env(env, "bye=no");

	TEST_ASSERT_EQUAL(1, index);

	index = get_index_env(env, "bye=yes");
	TEST_ASSERT_EQUAL(1, index);

	index = get_index_env(env, "not_found=yes");
	TEST_ASSERT_EQUAL(-1, index);

	// to read

	index = get_index_env(env, "bye");
	TEST_ASSERT_EQUAL(1, index);
	index = get_index_env(env, "not_found");
	TEST_ASSERT_EQUAL(-1, index);
	// key should not be found as it is not valid if using ft_strlen
	index = get_index_env(env, "bye=");
	TEST_ASSERT_EQUAL(1, index);
}

void	test_gkl() {
	int	key_len = get_key_len("bye");
	TEST_ASSERT_EQUAL(3, key_len);
	key_len = get_key_len("bye=");
	// int	index = get_index_env(env, "bye=no");
	TEST_ASSERT_EQUAL(3, key_len);
}
