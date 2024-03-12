#include "unity.h"
// this is a test for the support functions used in other tests

#include <stdlib.h>

#include "split_outside_quotes.c"
#include "utils.h"
#include "arr_utils.c"
#include "str_equal.c"
#include "expander.c"
#include "expand_variables.c"
#include "expand_var.c"
#include "interpret_quotes.c"

// get module support
#include "support_tokens.c"

int	export_run(t_shell *shell)
{
	// @audit leaks here (in arr_trim)
	add_pipes_as_tokens(shell);
	convert_split_token_string_array_to_tokens(shell);
	convert_tokens_to_string_array(shell->token);
	int	ret = builtin_export(shell, shell->token);
	destroy_all_tokens(shell);
	return (ret);
}

// export USER=
void	test_export_user() {
	t_shell	*shell = support_clean_env("export USER=", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export USER="@?/"
void	test_export_user_two() {
	t_shell	*shell = support_clean_env("export USER=\"@?/\"", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=@?/", NULL};
	export_run(shell);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export NEW="4 2"
void	test_export_new_one() {
	t_shell	*shell = support_clean_env("export NEW=\"4 2\"", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "NEW=4 2", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export NEW="4=2"
void	test_export_new_two() {
	t_shell	*shell = support_clean_env("export NEW=\"4=2\"", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "NEW=4=2", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export NEW=4=2
void	test_export_new_three() {
	t_shell	*shell = support_clean_env("export NEW=\"4=2\"", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "NEW=4=2", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export =
void	test_export_empty() {
	t_shell	*shell = support_clean_env("export =", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(1, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(shell->env));
	cleanup_support_test_token(shell);
	// export ''=''
	shell = support_clean_env("export ''=''", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	TEST_ASSERT_EQUAL(1, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(shell->env));
	cleanup_support_test_token(shell);
	// export ""=""
	shell = support_clean_env("export \"\"=\"\"", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	TEST_ASSERT_EQUAL(1, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(shell->env));
	cleanup_support_test_token(shell);
}


// export export -> print nothing (not a valid variable - not export without args!) @follow-up
void	test_export_keyword() {
	t_shell	*shell = support_clean_env("export export", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(shell->env));
	cleanup_support_test_token(shell);
}

void	test_export_invalid() {
	t_shell	*shell = support_clean_env("export TES\\~T=123", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(1, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(shell->env));
	cleanup_support_test_token(shell);

}

// export =============123
void	test_export_equals() {
	t_shell	*shell = support_clean_env("export =============123", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(1, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(shell->env));
	cleanup_support_test_token(shell);
}

// export echo
void	test_export_command() {
	t_shell	*shell = support_clean_env("export echo", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(shell->env));
	cleanup_support_test_token(shell);
}

// export NEW=42
void	test_export_number() {
	t_shell	*shell = support_clean_env("export NEW=42", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "NEW=42", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export PATH=tmp_path_test1:tmp_path_test2
void	test_export_path() {
	t_shell	*shell = support_clean_env("export PATH=tmp_path_test1:tmp_path_test2", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=tmp_path_test1:tmp_path_test2", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export PATH=tmp_path_test2:tmp_path_test1
void	test_export_path_reorder() {
	t_shell	*shell = support_clean_env("export PATH=tmp_path_test2:tmp_path_test1", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=tmp_path_test2:tmp_path_test1", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export 1TEST=
void	test_export_start_num_error() {
	t_shell	*shell = support_clean_env("export 1TEST=", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	TEST_ASSERT_EQUAL(1, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export TES=T=123
void	test_export_mult_equalsign() {
	t_shell	*shell = support_clean_env("export TES=T=123", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "TES=T=123", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}

// export TES$?T=123 @follow-up expansion: make this work
void	test_export_expanded_req_expansion() {
	t_shell	*shell = support_clean_env("export TES$?T=123", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	shell->env = append_str_arr_free(shell->env, ft_strdup("?=1"));
	char	**expected_env_start = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "?=1", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env_start, shell->env, arr_len(expected_env_start));
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "?=1", "TES1T=123", NULL};
	// this is not working yet because of the expansion not working correctly
	export_run(shell);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	TEST_ASSERT_EQUAL(0, export_run(shell));
	cleanup_support_test_token(shell);
}

// export $?=123
void	test_export_expanded_req_expansion_fail() {
	t_shell	*shell = support_clean_env("export $?=123", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	shell->env = append_str_arr_free(shell->env, ft_strdup("?=1"));
	char	**expected_env_start = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "?=1", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env_start, shell->env, arr_len(expected_env_start));
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "?=1", NULL};
	// this is not working yet because of the expansion not working correctly
	export_run(shell);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	// should print: minishell: export: `1=123': not a valid identifier
	TEST_ASSERT_EQUAL(1, export_run(shell));
	cleanup_support_test_token(shell);
}


// export ________=123
void	test_export_beginswith_underscore() {
	t_shell	*shell = support_clean_env("export ________=123", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	char	**expected_env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "________=123", NULL};
	TEST_ASSERT_EQUAL(0, export_run(shell));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_env, shell->env, arr_len(expected_env));
	cleanup_support_test_token(shell);
}
