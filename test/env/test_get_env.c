#include "unity.h"
#include "get_index.c"
#include "arr_utils.c"
#include "str_equal.c"
#include "support_lib.c"
#include "get_env.c"

void	test_get_env_returns_correct() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	**expected = arr_dup(env);
	char	*ret = get_env(expected, "key");
	TEST_ASSERT_EQUAL_STRING("forsure", ret);
	arr_free(expected);
	free(ret);
}

void	test_get_env_returns_correct_two() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	**expected = arr_dup(env);
	char	*ret = get_env(expected, "$key");
	TEST_ASSERT_NULL(ret);
	arr_free(expected);
}

void	test_get_env_returns_null() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = get_env(NULL, "$key");
	TEST_ASSERT_NULL(ret);
	ret = get_env(env, NULL);
	TEST_ASSERT_NULL(ret);
}
