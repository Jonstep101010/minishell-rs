#include "libft.h"
#include "unity.h"

#include "checks_basic.c"
#include "support_parser.h"
#include "build_tokens.c"
#include "expand_variables.c"

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
	arr_free(tokens);
}

void	test_only_expand() {
	char	*input = strdup("$somedir ");
	char	**tokens = split_outside_quotes(input, '|');

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	TEST_ASSERT_EQUAL_STRING("$somedir ", input);
	free(input);
	char	*expected[] =
		{"$somedir ", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
	arr_free(tokens);
}

void	test_leading_trailing_char() {
	char	*input = strdup("||echo $somedir|");
	char	**tokens = split_outside_quotes(input, '|');

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"echo $somedir", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
	arr_free(tokens);
	free(input);
}

void	test_leading_trailing_and_split() {
	char	*input = strdup("         echo $somedir' '           ");
	char	**tokens = split_outside_quotes(input, ' ');

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"echo", "$somedir' '", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 3);
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
	arr_free(tokens);
	free(input);
}

void	test_only_trim() {
	char	*input = strdup("                    ");
	char	**tokens = split_outside_quotes(input, ' ');

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
	arr_free(tokens);
	free(input);
}

void	test_only_trim_single() {
	char	*input = strdup("        h            ");
	char	**tokens = split_outside_quotes(input, ' ');

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"h", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
	arr_free(tokens);
	free(input);
}

void	test_only_trim_single_quotes() {
	char	*input = strdup("        '            ");
	char	**tokens = split_outside_quotes(input, ' ');

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"'", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
	arr_free(tokens);
	free(input);
}
