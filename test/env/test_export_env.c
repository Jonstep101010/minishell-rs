#include "unity.h"

#include "support_tokens.c"
#include "support_commands.c"
#include "support_msh.c"

void	test_replace_key() {
	t_shell	*shell = support_test_tokens((char *[]){"something=wrong", "this=false", "some=none", NULL});
	export_env(shell, ft_strdup("this=correct"));
	char	*expected = "this=correct";
	TEST_ASSERT_EQUAL_STRING(expected, shell->env[1]);
	cleanup_support_test_token(shell);
}

void	test_replace_key_two() {
	t_shell	*shell = support_test_tokens((char *[]){"something=wrong", "this=false", "some=none", NULL});
	tokenize(shell, "export this= correct");
	TEST_ASSERT_EQUAL_STRING("export", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("this=", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("correct", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_INT(0, builtin_export(shell, shell->token));
	TEST_ASSERT_EQUAL_STRING("this=", shell->env[1]);
	cleanup_support_test_token(shell);
}

void	test_export_invalid_name_one() {
	t_shell	*shell = support_test_tokens((char *[]){"something=wrong", "this=false", "some=none", NULL});
	tokenize(shell, "export th@is= correct");
	TEST_ASSERT_EQUAL_STRING("export", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("th@is=", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("correct", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_INT(1, builtin_export(shell, shell->token));
	TEST_ASSERT_EQUAL_STRING("this=false", shell->env[1]);
	cleanup_support_test_token(shell);
}

void	test_export_invalid_name_two() {
	t_shell	*shell = support_test_tokens((char *[]){"something=wrong", "this=false", "some=none", NULL});
	tokenize(shell, "export this==correct");
	TEST_ASSERT_EQUAL_STRING("this==correct", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_INT(1, builtin_export(shell, shell->token));
	TEST_ASSERT_EQUAL_STRING("this=false", shell->env[1]);
	cleanup_support_test_token(shell);
}

void	test_export_valid_name() {
	t_shell	*shell = support_test_tokens((char *[]){"something=wrong", "this=false", "some=none", NULL});
	tokenize(shell, "export this=correct that=false");
	TEST_ASSERT_EQUAL_STRING("this=correct", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("that=false", shell->token[0].cmd_args[2].elem);
	builtin_export(shell, shell->token);
	TEST_ASSERT_EQUAL_INT(0, builtin_export(shell, shell->token));
	TEST_ASSERT_EQUAL_STRING("this=correct", shell->env[1]);
	TEST_ASSERT_EQUAL_STRING("that=false", shell->env[6]);
	cleanup_support_test_token(shell);
}

void	test_replace_using_update_ez() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup(env);
	update_var(arr, ft_strdup("something=correct"));
	char	*expected[] = {"something=correct", "this=false", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
	arr_free(arr);
}

void	test_replace_using_update_mid() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup(env);
	update_var(&arr[1], ft_strdup("this=correct"));
	char	*expected[] = {"something=wrong", "this=correct", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
	arr_free(arr);
}

void	test_replace_using_update_end() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup(env);
	update_var(&arr[2], ft_strdup("some=something"));
	char	*expected[] = {"something=wrong", "this=false", "some=something", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
	arr_free(arr);
}

void	test_replace_using_update_null() {
	char	*env[] = {"something=wrong", "this=false", "some=none", NULL};
	char	**arr = arr_dup(env);
	char	*new = ft_strdup("some=something");
	update_var(&arr[3], new);
	char	*expected[] = {"something=wrong", "this=false", "some=none", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 4);
	arr_free(arr);
	free(new);
}

// // we can only see if the printing from the function is correct:
// // minishell: fatal: invalid memory!
// // void	test_export_with_invalid_env() {
// // 	t_shell	*shell = support_test_tokens("export this=correct that=false", (char *[]){"something=wrong", "this=false", "some=none", NULL});
// //
// // 	tokenize(shell);
// // 	TEST_ASSERT_EQUAL_STRING("this=correct", shell->token[0].cmd_args[1].elem);
// // 	TEST_ASSERT_EQUAL_STRING("that=false", shell->token[0].cmd_args[2].elem);
// // 	arr_free(shell->env);
// // 	shell->env = NULL;
// // 	export(shell, shell->token);
// // }
