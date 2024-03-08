#include "unity.h"
#include "support_lib.c"

void	test_can_join_multiple_strings(void)
{
	char	*joined;

	joined = str_join(3, "hello", " ", "world");
	TEST_ASSERT_NOT_NULL(joined);
	TEST_ASSERT_EQUAL_STRING("hello world", joined);
	free(joined);
}

void	test_can_join_zero(void)
{
	char	*joined;

	joined = str_join(0, "hello", " ", "world");
	TEST_ASSERT_NULL(joined);
}

void	test_can_join_null(void)
{
	char	*joined;

	joined = str_join(INT_MAX, NULL);
	TEST_ASSERT_NULL(joined);
}
