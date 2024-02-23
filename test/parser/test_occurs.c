#include "unity.h"
#include "occurs.c"
#include "support_lib.c"

void	test_returns_null() {
	TEST_ASSERT_NULL(occurs(NULL, "echo "));
	TEST_ASSERT_NULL(occurs("somestring", "echo "));
	TEST_ASSERT_NULL(occurs("echo ", NULL));
	TEST_ASSERT_NULL(occurs(NULL, NULL));
}

void	test_should_find() {
	char	*ret = occurs("echo command string ls", "echo");
	TEST_ASSERT_NOT_NULL(ret);
	TEST_ASSERT_EQUAL_STRING("echo command string ls", ret);
}

void	test_should_find2() {
	char	*ret = occurs("echo command string ls", "string");
	TEST_ASSERT_NOT_NULL(ret);
	TEST_ASSERT_EQUAL_STRING("string ls", ret);
}

void	test_should_trigger() {
	char	*ret = occurs("echoexit", "echo");
	TEST_ASSERT_NOT_NULL(ret);
	TEST_ASSERT_EQUAL_STRING("echoexit", ret);
	char	*ret2 = occurs("exitecho", "echo");
	TEST_ASSERT_NOT_NULL(ret2);
	TEST_ASSERT_EQUAL_STRING("echo", ret2);
}

void	test_occurs_exclusive() {
	char	*ret = occurs_exclusively("", "");
	TEST_ASSERT_NULL(ret);
	char	*ret2 = occurs_exclusively("echo", "echoexit");
	TEST_ASSERT_NULL(ret2);
	char	*ret3 = occurs_exclusively("echo", "exitecho");
	TEST_ASSERT_NULL(ret3);
	char	*ret4 = occurs_exclusively("echo", "echo");
	TEST_ASSERT_EQUAL_STRING(ret4, "echo");
}

void	test_occurs_exclusive_two() {
	char	*ret = occurs_exclusively("hello", "heyho");
	TEST_ASSERT_NULL(ret);
	char	*ret2 = occurs_exclusively("echo", "echoecho");
	TEST_ASSERT_NULL(ret2);
}
