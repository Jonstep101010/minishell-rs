#include "unity.h"
#include "str_equal.c"
#include "support_lib.c"

void	test_occurs_exclusive() {
	char	*ret = equal("", "");
	TEST_ASSERT_NULL(ret);
	char	*ret2 = equal("echo", "echoexit");
	TEST_ASSERT_NULL(ret2);
	char	*ret3 = equal("echo", "exitecho");
	TEST_ASSERT_NULL(ret3);
	char	*ret4 = equal("echo", "echo");
	TEST_ASSERT_EQUAL_STRING(ret4, "echo");
}

void	test_occurs_exclusive_two() {
	char	*ret = equal("hello", "heyho");
	TEST_ASSERT_NULL(ret);
	char	*ret2 = equal("echo", "echoecho");
	TEST_ASSERT_NULL(ret2);
}
