#include "add_var.c"
#include "find_key.c"
#include "unity.h"
#include "arr_utils.c"
#include "occurs.c"
#include "libft.h"
#include "../include/libft/src/string/str_cchr.c"

#include "print_arr_sep.c"

void	test_replace_key() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup((const char **)env);
	TEST_ASSERT_EQUAL_STRING_ARRAY(env, arr, 4);
	if (!(arr = add_env(arr, "this=correct")))
		TEST_FAIL();
	char	*expected[] = {"something=wrong", "this=correct", "some=none", NULL};
	printf("%s\n", arr[1]);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
}

void	test_replace_key_two() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup((const char **)env);
	add_env(arr, "this= correct");
	char	*expected[] = {"something=wrong", "this= correct", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
}
