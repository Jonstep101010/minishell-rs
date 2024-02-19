#include "find_key.c"
#include "unity.h"
#include "arr_utils.c"
#include "occurs.c"

#include "print_arr_sep.c"

#include "free_strjoin.c"

void	test_get_key_len() {
	char	*s = "KEY=val";
	TEST_ASSERT_EQUAL(3, get_key_len(s));
}
void	test_get_key_len_two() {
	char	*s = "V==val";
	TEST_ASSERT_EQUAL(1, get_key_len(s));
}

void	test_get_key_error() {
	char	*s = "KEYval";
	TEST_ASSERT_EQUAL(-1, get_key_len(s));
}

void	test_get_key_error_two() {
	char	*s = "=Vval";
	TEST_ASSERT_EQUAL(-1, get_key_len(s));
}

void	test_find_key() {
	TEST_ASSERT_EQUAL(3, get_key_len("bye=yes"));
	char	*arr[] = {"hello=wrong", "bye=yes", "night=wrong", NULL};
	int	index = find_key_env((const char **)arr, "bye=no", get_key_len);
	TEST_ASSERT_EQUAL(1, index);
}

void	test_find_key_error() {
	TEST_ASSERT_EQUAL(4, get_key_len("byes=wrong"));
	char	*arr[] = {"hello=wrong", "byes=wrong", "night=wrong", NULL};
	int	index = find_key_env((const char **)arr, "bye=no", get_key_len);
	TEST_ASSERT_EQUAL(-1, index);
}

void	test_get_len_until() {
	char	*s = "hello=world";
	TEST_ASSERT_EQUAL(5, get_len_until(s, '='));

	char	*s2 = "$key$";
	TEST_ASSERT_EQUAL(0, get_len_until(s2, '$'));
	TEST_ASSERT_EQUAL(3, get_len_until(&s2[1], '$'));

	// int i = 1;
	// while (s2[i] && i <= 3)
	// {
	// 	printf("%c\n", s2[i]);
	// 	i++;
	// }
}
