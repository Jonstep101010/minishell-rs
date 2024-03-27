#include "support_lib.c"
#include "unity.h"
#include "minishell.h"
#include <stdbool.h>

// void test_wrapper_both()
// {
// 	int i = 0;
// 	int iftrue = 1;
// 	int *index = &i;
// 	char *str = "hello1234";

// 	// Test while_d
// 	TEST_ASSERT_EQUAL_INT(0, while_d(str, &ft_isalnum, iftrue, index));
// 	TEST_ASSERT_EQUAL_INT(0, *index);

// 	// Test while_i
// 	*index = 0; // Reset index
// 	TEST_ASSERT_EQUAL_INT(0, while_i(str, &ft_isalnum, iftrue, index));
// 	TEST_ASSERT_EQUAL_INT(0, *index);
// }

// void test_wrapper()
// {
// 	int i = 0;
// 	int iftrue = 1;
// 	int *index = &i;
// 	TEST_ASSERT(while_d("hello1234", &ft_isalnum, iftrue, index) == 0);
// 	TEST_ASSERT(while_i("hello1234", &ft_isalnum, iftrue, index) == 0);
// }

// void test_while_wrapper()
// {
// 	int i = 0;
// 	int iftrue = 1;
// 	int *index = &i;
// 	char *str = "hello1234";

// 	// Test while_wrapper
// 	TEST_ASSERT_EQUAL_INT(0, while_i(str, &ft_isalnum, iftrue, index));
// 	TEST_ASSERT_EQUAL_INT(0, *index);
// }
