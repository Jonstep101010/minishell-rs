#include "unity.h"
#include "support_tokens.c"
#include "support_commands.c"
#include "support_msh.c"

void	test_builtin_unset_one() {
	t_shell	*shell = support_test_tokens((char *[]){
		"USER=vscode", "val=true", NULL});
	tokenize(shell, "unset USER==== val");
	TEST_ASSERT_EQUAL(1, builtin_unset(shell, shell->token));
	TEST_ASSERT_EQUAL_STRING("USER=vscode", shell->env[0]);
	TEST_ASSERT_EQUAL_STRING("val=true", shell->env[1]);
	TEST_ASSERT_EQUAL(1, get_index_env(shell->env, "val"));
	cleanup_support_test_token(shell);
}

void	test_builtin_unset_two() {
	t_shell	*shell = support_test_tokens((char *[]){
		"USER=vscode", "val=true", NULL});
	tokenize(shell, "unset ----- val");
	char	**expected = arr_dup(shell->env);
	TEST_ASSERT_EQUAL(1, builtin_unset(shell, shell->token));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, shell->env, arr_len(shell->env));
	TEST_ASSERT_EQUAL(1, get_index_env(shell->env, "val"));
	arr_free(expected);
	cleanup_support_test_token(shell);
}

void	test_remove_key_value() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup(env);
	unset_internal(((const char *[]){"unset", "this", NULL}), arr);
	if (!arr)
		TEST_FAIL();
	char	*expected[] = {"something=wrong", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 3);
	arr_free(arr);
}
