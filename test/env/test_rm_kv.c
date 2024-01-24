#include "libft.h"
#include "rm_var.c"
#include "unity.h"
#include "arr_utils.c"
#include "occurs.c"
#include "find_key.c"
#include "env.h"

void	test_remove_key_value() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup((const char **)env);
	char	**tmp = rm_env(arr, "this");
	TEST_ASSERT_EQUAL(4, ft_strlen("this"));
	if (!tmp)
		TEST_FAIL();
	arr = tmp;
	char	*expected[] = {"something=wrong", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 3);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
}
