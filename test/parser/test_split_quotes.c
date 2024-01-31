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
