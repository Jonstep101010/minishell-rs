#include "environment.h"
#include "get_index.c"
#include "unity.h"
#include "arr_utils.c"
#include "str_equal.c"
#include "support_lib.c"
#include "print_arr_sep.c"
#include "export_env.c"
#include "get_env.c"
#include "expand_var.c"
#include "builtin_exit.c"
#include "destroy_tokens.c"
#include "error.c"


// @audit see if this function is even required for the spec
void	test_read_returns_correct() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	**expected = arr_dup(env);
	char	*ret = get_env(expected, "key");
	TEST_ASSERT_EQUAL_STRING("forsure", ret);
	arr_free(expected);
	free(ret);
}

void	test_read_returns_correct_two() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	**expected = arr_dup((char **)env);
	char	*ret = expand_var("ke", (char **)expected);
	TEST_ASSERT_EQUAL_STRING("ke", ret);
	free(ret);
	ret = expand_var("$ke", expected);
	TEST_ASSERT_EQUAL_STRING("", ret);
	free(ret);
	ret = expand_var("$key", expected);
	TEST_ASSERT_EQUAL_STRING("forsure", ret);
	arr_free(expected);
	free(ret);
}

void	test_expand_var() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$keylong", env);
	TEST_ASSERT_EQUAL_STRING("", ret);
	// @todo "" instead of forsurelong make this pass
	free(ret);
}

// return following variables for expansion
void	test_expand_var_dollarsign() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", "ong=true", NULL};
	char	*ret = expand_var("$keyl$ong", env);
	TEST_ASSERT_EQUAL_STRING("$ong", ret);

	free(ret);
}

void	test_expand_var_prefix_dollarsign() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$keylong", env);
	TEST_ASSERT_EQUAL_STRING("", ret);

	free(ret);
}

void	test_expand_var_prefix_no_match() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$ke", env);
	TEST_ASSERT_EQUAL_STRING("", ret);
	free(ret);
}

void	test_expand_var_prefix_dollarsign_null() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$key$", env);
	TEST_ASSERT_EQUAL_STRING("forsure$", ret);
	free(ret);
}

void	test_expand_var_prefix_following_null() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	*ret = expand_var("$key$not$", env);
	TEST_ASSERT_EQUAL_STRING("forsure$not$", ret);
	free(ret);
}

// // // void	test_expand_var_prefix_multiple() {
// // // 	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
// // // 	char	*ret = expand_var("$$key$notmine", env);
// // // 	TEST_ASSERT_EQUAL_STRING("$$forsure$notmine", ret);
// // // 	// trim off until non-expanded key
// // // 	free(ret);
// // // }
