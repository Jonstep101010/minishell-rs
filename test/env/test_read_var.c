#include "environment.h"
#include "key.c"
#include "unity.h"
#include "arr_utils.c"
#include "str_equal.c"
#include "support_lib.c"
#include "print_arr_sep.c"
#include "export_var.c"
#include "export.c"
#include "get_env_var.c"
#include "expand_var.c"
#include "env.c"
#include "free_strjoin.c"
#include "error.c"

void	test_read_returns_correct() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	**expected = arr_dup((const char **)env);
	char	*ret = get_env_var(expected, "key");
	TEST_ASSERT_EQUAL_STRING("forsure", ret);
	arr_free(expected);
	free(ret);
}

void	test_read_returns_correct_two() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	**expected = arr_dup((const char **)env);
	char	*ret = expand_var("ke", (const char **)expected);
	TEST_ASSERT_EQUAL_STRING("ke", ret);
	free(ret);
	ret = expand_var("$ke", (const char **)expected);
	TEST_ASSERT_EQUAL_STRING("", ret);
	free(ret);
	ret = expand_var("$key", (const char **)expected);
	TEST_ASSERT_EQUAL_STRING("forsure", ret);
	arr_free(expected);
	free(ret);
}

void	test_expand_var() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$keylong", (const char **)env);
	TEST_ASSERT_EQUAL_STRING("forsurelong", ret);

	free(ret);
}

void	test_expand_var_dollarsign() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$keyl$ong", (const char **)env);
	TEST_ASSERT_EQUAL_STRING("forsurel$ong", ret);

	free(ret);
}

void	test_expand_var_prefix_dollarsign() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$keylong", (const char **)env);
	TEST_ASSERT_EQUAL_STRING("forsurelong", ret);

	free(ret);
}

void	test_expand_var_prefix_no_match() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$ke", (const char **)env);
	TEST_ASSERT_EQUAL_STRING("", ret);
	free(ret);
}

void	test_expand_var_prefix_dollarsign_null() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$key$", (const char **)env);
	TEST_ASSERT_EQUAL_STRING("forsure$", ret);
	free(ret);
}

// void	test_expand_var_prefix_multiple() {
// 	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
// 	char	*ret = expand_var("$$key$notmine", (const char **)env);
// 	TEST_ASSERT_EQUAL_STRING("$$forsure$notmine", ret);
// 	// trim off until non-expanded key
// 	free(ret);
// }
