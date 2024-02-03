#include "export.c"
#include "find_key.c"
#include "struct.h"
#include "unity.h"
#include "arr_utils.c"
#include "occurs.c"
#include "libft.h"
#include "../include/libft/src/string/str_cchr.c"

#include "print_arr_sep.c"

#include "support_shell.c"

void	test_replace_key() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup((const char **)env);
	char	**ret = export_var(arr, "this=correct");
	if (!ret)
		TEST_FAIL();
	char	*expected[] = {"something=wrong", "this=correct", "some=none", NULL};
	printf("%s\n", arr[1]);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, ret, 3);
	arr_free(ret);
}

void	test_replace_key_two() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup((const char **)env);
	char	**ret = export_var(arr, "this= correct");
	if (!ret || !arr)
		TEST_FAIL();
	char	*expected[] = {"something=wrong", "this= correct", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, ret, 4);
	arr_free(ret);
}

void	test_export_invalid_name() {
	t_shell	*shell = setup_shell(NULL);
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup((const char **)env);
	shell->owned_envp = arr;
	char	*tmp[] = {"export", "th@is=", "correct", NULL};
	shell->command = arr_dup((const char **)tmp);
	export(shell);
	char	*expected[] = {"something=wrong", "this=false", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
	arr_free(arr);
	arr_free(shell->command);
	clean_shell(shell);
}
