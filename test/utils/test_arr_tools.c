#include "unity.h"
#include "support_lib.c"

// TEST_INCLUDE_PATH("../include/libft/src/")

void	test_arr_len() {
	char	**arr = (char *[]){"0", "1", "2", "3", "4", NULL};
	TEST_ASSERT_NOT_NULL(arr);
	TEST_ASSERT_EQUAL(5, arr_len((char **)arr));
}

void	test_arr_dup() {
	char	*arr[] = {"0", "1", "2", "3", "4", NULL};
	TEST_ASSERT_NOT_NULL(arr);
	char	**copy = arr_dup(arr);
	TEST_ASSERT_NOT_NULL(copy);
	TEST_ASSERT_EQUAL(5, arr_len(copy));
	TEST_ASSERT_EQUAL_STRING_ARRAY(arr, copy, 6);
	arr_free(copy);
}

void	test_arr_null() {
	char	**arr = NULL;
	TEST_ASSERT_NULL(arr);
	TEST_ASSERT_EQUAL(0, arr_len(arr));
	char	**actual = arr_dup(arr);
	TEST_ASSERT_NULL(actual);
	arr_free(actual);
}

void	test_arr_free() {
	char	**arr = NULL;
	TEST_ASSERT_NULL(arr);
	char	**actual = (char **)calloc(1, sizeof(char *));
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_NULL(actual[0]);
	char	**copy = arr_dup(actual);
	TEST_ASSERT_NOT_NULL(copy);
	TEST_ASSERT_NULL(copy[0]);
	arr_free(actual);
	arr_free(copy);
	TEST_ASSERT_NULL(arr_dup(NULL));
}

// #include "print_arr_sep.c"
#include "str_equal.c"
#include "arr_utils.c"

void	test_arr_trim_one(void)
{
	char	**arr = (char *[]){"  ls -l somedir ", " cat -e ", " wc -l", NULL};
	char	**expected = (char *[]){"ls -l somedir", "cat -e", "wc -l", NULL};
	char	**ret = arr_trim(arr, " ");
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, ret, 4);
	arr_free(ret);
}

void	test_arr_trim_two(void)
{
	char	**arr = (char *[]){"  ls -l somedir ", " cat -e ", " wc -l", NULL};
	char	**expected = (char *[]){"ls -l somedir", "cat -e", "wc -l", NULL};
	char	**ret = arr_trim(arr, " ");
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, ret, 4);
	arr_free(ret);
}

void	test_strncmp_one(void)
{
	TEST_ASSERT_GREATER_OR_EQUAL(1, ft_strncmp("hello", "gello", 5));
	TEST_ASSERT_NOT_EQUAL(0, ft_strncmp("gello", "hello", 5));
}

// void	test_arr_ncmp_one(void)
// {
// 	const char	*arr[] = {"hello", "world", "!", NULL};
// 	const char	*arr2[] = {"hello", "world", "!", NULL};
// 	TEST_ASSERT_EQUAL(0, arr_ncmp(arr, arr2, arr_len(arr)));
// }

// void	test_arr_ncmp_two(void)
// {
// 	// check with different lengths
// 	const char	*arr[] = {"hello", "world", "!", NULL};
// 	const char	*arr2[] = {"hello", "world", "!", "!", NULL};
// 	TEST_ASSERT_NOT_EQUAL(0, arr_ncmp(arr, arr2, arr_len(arr)));
// 	// check with invalid array
// 	TEST_ASSERT_NOT_EQUAL(0, arr_ncmp(arr, NULL, arr_len(arr)));
// 	// check with invalid string inside array
// 	const char	*arr3[] = {"hello", "world", "!", "!", NULL};
// 	TEST_ASSERT_NOT_EQUAL(0, arr_ncmp(arr, arr3, arr_len(arr)));
// }

void	test_arr_dup_empty_string() {
	char **arr = arr_dup((char *[]){"", NULL});
	TEST_ASSERT_NOT_NULL(arr);
	TEST_ASSERT_EQUAL_STRING("", arr[0]);
	arr_free(arr);
}

// char	**init_env(const char **envp)
// {
// 	char	**env;

// 	if (!envp)
// 		return (NULL);
// 	env = append_str_arr(envp, "?=0");
// 	if (!env)
// 		return (NULL);
// 	return (env);
// }

// void	test_arr_init_env() {
// 	char	*envp[] = {"PATH=/usr/bin", "HOME=/home", "USER=me", NULL};
// 	char	**env = init_env((const char **)envp);

// 	TEST_ASSERT_NOT_NULL(env);
// 	char	*expected[] = {"PATH=/usr/bin", "HOME=/home", "USER=me", "?=0", NULL};
// 	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, env, 5);
// 	arr_free(env);
// }

void	test_append_arr_free() {
	char	**arr = NULL;
	char	*str = strdup("hello");
	char	**ret = append_str_arr_free(arr, str);
	TEST_ASSERT_NOT_NULL(ret);
	TEST_ASSERT_EQUAL_STRING("hello", ret[0]);
	arr_free(ret);
}

void	test_append_arr_free_one() {
	char	**arr = NULL;
	char	*str = NULL;
	char	**ret = append_str_arr_free(arr, str);
	TEST_ASSERT_NULL(ret);
	arr_free(ret);
}
