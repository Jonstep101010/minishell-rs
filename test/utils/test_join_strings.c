#include "free_strjoin.c"
#include "unity.h"
#include <string.h>
#include "support_lib.c"

void test_join_multiple_strings() {
	char	*s1 = strdup("hello");
	char	*s2 = strdup("world");
	char	*s3 = strdup("!");

	char	*joined = free_strjoin(3, s1, s2, s3);

	TEST_ASSERT_EQUAL_STRING("helloworld!", joined);
	free(joined);
}

void test_join_null() {
	char	*s1 = strdup("hello");
	char	*s2 = strdup("world");
	char	*s3 = NULL;

	char	*joined = free_strjoin(3, s1, s2, s3);

	TEST_ASSERT_EQUAL_STRING(NULL, joined);
}

void test_join_null_two() {
	char	*s1 = NULL;
	char	*s2 = strdup("world");
	char	*s3 = NULL;

	char	*joined = free_strjoin(3, s1, s2, s3);

	TEST_ASSERT_EQUAL_STRING(NULL, joined);
}

void test_join_null_three() {
	char	*s1 = strdup("hello");
	char	*s2 = strdup("world");
	char	*s3 = NULL;

	char	*joined = free_strjoin(3, s1, s2, s3);

	TEST_ASSERT_EQUAL_STRING(NULL, joined);
}

void test_join_null_four() {
	char	*s1 = NULL;
	char	*s2 = strdup("world");
	char	*s3 = strdup("!");

	char	*joined = free_strjoin(3, s1, s2, s3);

	TEST_ASSERT_EQUAL_STRING(NULL, joined);
}

void	test_free_multiple_items() {
	char	*s1 = strdup("hello");
	char	*s2 = strdup("world");
	char	*s3 = strdup("!");

	free_n(3, s1, s2, s3);
}
