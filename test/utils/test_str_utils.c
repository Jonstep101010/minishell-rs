#include "str_utils.c"
#include "unity.h"
#include <string.h>
#include "support_lib.c"

void	test_free_first_join_no_leaks() {
	char	*s1 = strdup("Hello");
	const char	*s2 = ", ";
	const char	*s3 = "World!";

	TEST_ASSERT_NOT_NULL(s1);
	char	*joined = free_first_join(s1, s2);
	TEST_ASSERT_NOT_NULL(joined);
	TEST_ASSERT_EQUAL_STRING("Hello, ", joined);
	joined = free_first_join(joined, s3);
	TEST_ASSERT_EQUAL_STRING("Hello, World!", joined);
	free(joined);
}
