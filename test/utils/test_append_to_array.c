#include "unity.h"

#include "arr_utils.c"
#include "str_equal.c"
#include "support_lib.c"
#include "utils.h"

void	test_append_once() {
	char	*tmp[] = {"Hello, ", "World", NULL};
	char	*expected[] = {"Hello, ", "World", "!", NULL};
	char	**arr = arr_dup(tmp);
	TEST_ASSERT_NOT_NULL(arr);
	TEST_ASSERT_EQUAL_STRING_ARRAY(tmp, arr, 3);
	TEST_ASSERT_NULL(arr[2]);
	char	**appended = append_str_arr_free(arr, strdup("!"));

	TEST_ASSERT_NOT_NULL(appended);
	TEST_ASSERT_NULL(appended[3]);
	TEST_ASSERT_EQUAL_STRING("!", appended[2]);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, appended, 4);

	TEST_ASSERT_EQUAL(arr_len(appended), arr_len(expected));
	arr_free(appended);
}

void	test_append() {
	char	**arr = append_str_arr((char *[]){"Hello, ", "World", NULL}, "!");
	char	*expected[] = {"Hello, ", "World", "!", "my", "precious", NULL};

	TEST_ASSERT_NULL(expected[5]);
	char	**appended1 = append_str_arr(arr, "my");
	TEST_ASSERT_NOT_NULL(appended1);
	arr_free(arr);
	TEST_ASSERT_NULL(appended1[4]);

	char	**appended2 = append_str_arr(appended1, "precious");
	arr_free(appended1);
	TEST_ASSERT_NULL(appended2[5]);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, appended2, 6);
	arr_free(appended2);
}

void	test_append_after_rm() {
	char	*tmp[] = {"World", NULL, NULL};
	char	**arr = append_str_arr((tmp), "Yapping");
	TEST_ASSERT_NOT_NULL(arr);
	char	*expected[] = {"World", "Yapping", NULL, NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 3);
	TEST_ASSERT_EQUAL(arr_len(expected), arr_len(arr));
	rm_str_arr(arr, "World");
	rm_str_arr(arr, "Yapping");
	char	*expected2[] = {NULL, NULL, NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected2, arr, 3);

	TEST_ASSERT_NOT_NULL(arr);
	char	**appended1 = append_str_arr(arr, "Hello, ");
	arr_free(arr);
	TEST_ASSERT_NOT_NULL(appended1);
	TEST_ASSERT_NOT_NULL(appended1[0]);
	char	**appended2 = append_str_arr(appended1, "World!");
	TEST_ASSERT_NOT_NULL(appended2);
	arr_free(appended1);

	char	*expected3[] = {"Hello, ", "World!", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected3, appended2, 3);
	arr_free(appended2);
}

void	test_create_null() {
	char	**arr = append_str_arr(NULL, "Hello");
	char	*expected[] = {"Hello", NULL};
	TEST_ASSERT_NOT_NULL(arr);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, arr, 2);
	char	**nullret = append_str_arr(arr, NULL);
	TEST_ASSERT_NULL(nullret);
	arr_free(arr);
}
