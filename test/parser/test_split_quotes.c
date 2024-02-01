#include "libft.h"
#include "unity.h"

#include "support_parser.h"

void	test_find_leaks() {
	char	*input = strdup("echo | \"nopipes |\" | echo hello");
	char	**tokens = split_outside_quotes(input, '|');

	if (!tokens || !input)
		TEST_FAIL();
	TEST_ASSERT_EQUAL_STRING("echo | \"nopipes |\" | echo hello", input);
	free(input);
	char	*expected[] =
		{"echo ", " \"nopipes |\" ", " echo hello", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 4);
	arr_free(tokens);
}

void	test_find_leaks_two() {
	char	*input = strdup("echo");
	char	**tokens = split_outside_quotes(input, '|');

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	TEST_ASSERT_EQUAL_STRING("echo", input);
	free(input);
	char	*expected[] =
		{"echo", NULL, NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
	print_arr(tokens);
	arr_free(tokens);
}
