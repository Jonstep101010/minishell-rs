#include "add_var.c"
#include "env.h"
#include "find_key.c"
#include "unity.h"
#include "arr_utils.c"
#include "occurs.c"
#include "libft.h"
#include "../include/libft/src/string/str_cchr.c"

#include "read_var.c"

#include "print_arr_sep.c"

void	test_read_returns_correct() {
	char	*env[] = {"not=looking", "maybe=?looking", "key=forsure", "notmine=(null)", NULL};
	char	**expected = arr_dup((const char **)env);
	char	*ret = get_var_val((const char **)expected, "key");
	TEST_ASSERT_EQUAL_STRING("forsure", ret);
}
