#include "unity.h"
#include "support_tokens.c"
#include "support_msh.c"
#include "bin_path.c"

void	test_get_path() {
	t_shell	*shell = support_clean_env((char *[]){"PATH=/usr/bin:/sbin:/bin", NULL});
	char	**expected = (char *[]){"PATH=/usr/bin:/sbin:/bin", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, shell->env, 2);
	char	*ret = get_env(shell->env, "PATH");
	TEST_ASSERT_EQUAL_STRING("/usr/bin:/sbin:/bin", ret);
	free(ret);
	cleanup_support_test_token(shell);
}

void	test_get_bin_segv() {
	t_shell	*shell = support_clean_env((char *[]){"PATH=/usr/bin:/sbin:/bin", NULL});
	const char	*path = get_env(shell->env, "PATH");
	const char	**split_path = (const char **) ft_split(path, ':');

	TEST_ASSERT_EQUAL_STRING("/usr/bin:/sbin:/bin", path);
	arr_free((char **)split_path);
	free((char *)path);
	cleanup_support_test_token(shell);
}

void	test_get_bin_addr() {
	t_shell	*shell = support_clean_env((char *[]){"PATH=/usr/bin:/sbin:/bin", NULL});
	const char	*path = get_env(shell->env, "PATH");
	shell->token = tokenize(shell, "ls");
	TEST_ASSERT_NULL(shell->token->bin);
	TEST_ASSERT_EQUAL_INT(0, set_binpath(shell->env, "ls", &shell->token->bin));
	TEST_ASSERT_EQUAL_STRING("/bin/ls", shell->token->bin);
	free((char *)path);
	cleanup_support_test_token(shell);
}

void	test_get_bin_path_null() {
	t_shell	*shell = support_clean_env((char *[]){"nonexistent=path", NULL});
	TEST_ASSERT_NULL(get_env(shell->env, "PATH"));
	shell->token = tokenize(shell, "ls");
	TEST_ASSERT_NULL(shell->token->bin);
	set_binpath(shell->env, "ls", &shell->token->bin);
	TEST_ASSERT_EQUAL_STRING("/bin/ls", shell->token->bin);
	cleanup_support_test_token(shell);
}

void	test_no_leaks_null() {
	char	*dst = NULL;
	get_bin(NULL, "ls", &dst);
	TEST_ASSERT_EQUAL_STRING("/bin/ls", dst);
	free_null(&dst);
}
