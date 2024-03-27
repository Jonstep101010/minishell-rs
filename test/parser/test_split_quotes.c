#include "support_lib.c"
#include "tokens.h"
#include "unity.h"

#include "interpret_quotes.c"
// #include "split_outside_quotes.c"
#include "libft.h"
#include "utils.h"
#include "arr_utils.h"
#include <unistd.h>
#include "parser.h"

#include "expander.c"
#include "get_index.c"
#include "get_env.c"
#include "error.c"
#include "arr_utils.c"
#include "str_equal.c"

#include "split_outside_quotes.c"
#include "destroy_tokens.c"

void	test_find_leaks() {
	char	*input = strdup("echo | \"nopipes |\" | echo hello");
	char	**tokens = split_outside_quotes(input, "|");

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
	char	**tokens = split_outside_quotes(input, "|");

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	TEST_ASSERT_EQUAL_STRING("echo", input);
	free(input);
	char	*expected[] =
		{"echo", NULL, NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
}

void	test_only_expand() {
	char	*input = strdup("$somedir ");
	char	**tokens = split_outside_quotes(input, "|");

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	TEST_ASSERT_EQUAL_STRING("$somedir ", input);
	free(input);
	char	*expected[] =
		{"$somedir ", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
}

void	test_leading_trailing_char() {
	char	*input = strdup("||echo $somedir|");
	char	**tokens = split_outside_quotes(input, "|");

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"echo $somedir", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

void	test_leading_trailing_and_split() {
	char	*input = strdup("         echo $somedir' '           ");
	char	**tokens = split_outside_quotes(input, WHITESPACE);

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"echo", "$somedir' '", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 3);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

void	test_only_trim() {
	char	*input = strdup("                    ");
	char	**tokens = split_outside_quotes(input, WHITESPACE);

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

void	test_only_trim_single() {
	char	*input = strdup("        h            ");
	char	**tokens = split_outside_quotes(input, WHITESPACE);

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"h", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

void	test_only_trim_single_quotes() {
	char	*input = strdup("        '            ");
	char	**tokens = split_outside_quotes(input, WHITESPACE);

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"'", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

void	test_only_trim_single_quotes2() {
	char	*input = strdup("        'h'            ");
	char	**tokens = split_outside_quotes(input, WHITESPACE);

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"'h'", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

void	test_set_of_chars_isspace() {
	char	*input = strdup("    \t \r   h   \n  \t  \r  \n  ");
	char	**tokens = split_outside_quotes(input, " \t\r\n");

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"h", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

void	test_set_of_chars_isspace_error() {
	char	*input = strdup("    \t \r   h   \n  \t  \r  \n  ");
	char	**tokens = split_outside_quotes(input, " \t\r\n");

	if (!input)
		TEST_FAIL_MESSAGE("input alloc");
	if (!tokens)
		TEST_FAIL_MESSAGE("tokens alloc");
	char	*expected[] =
		{"h", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 2);
	TEST_ASSERT_EQUAL(arr_len(tokens), arr_len(expected));
	arr_free(tokens);
	free(input);
}

#ifndef WHITESPACE
# define WHITESPACE " \t\r\n\v\f"
#endif

void	test_split_some_stuff() {
	char *line = "ls \n-l\r \tsomedir | cat -e | wc -l";
	char	**split_tokens = split_outside_quotes(line, "|");
	char	**expected = (char *[]){"ls \n-l\r \tsomedir ", " cat -e ", " wc -l", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, split_tokens, 4);

	// do string trim on all spaces inside the split tokens
	char	**trimmed_tokens = arr_trim(split_tokens, WHITESPACE);
	char	**expected_trimmed = (char *[]){"ls \n-l\r \tsomedir", "cat -e", "wc -l", NULL};
	TEST_ASSERT_NOT_NULL(trimmed_tokens);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_trimmed, trimmed_tokens, 4);

	char	**split_tokens_0 = split_outside_quotes(trimmed_tokens[0], WHITESPACE);

	char	**expected_0 = (char *[]){"ls", "-l", "somedir", NULL};

	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_0, split_tokens_0, 4);

	arr_free(trimmed_tokens);
	arr_free(split_tokens);
	arr_free(split_tokens_0);
}

void	test_nothing_to_trim() {
	char *line = "ls -l somedir | cat -e | wc -l";
	char	**split_tokens = split_outside_quotes(line, "|");
	char	**expected = (char *[]){"ls -l somedir ", " cat -e ", " wc -l", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, split_tokens, 4);

	// do string trim on all spaces inside the split tokens
	char	**trimmed_tokens = arr_trim(split_tokens, WHITESPACE);
	char	**expected_trimmed = (char *[]){"ls -l somedir", "cat -e", "wc -l", NULL};
	TEST_ASSERT_NOT_NULL(trimmed_tokens);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_trimmed, trimmed_tokens, 4);

	char	**split_tokens_0 = split_outside_quotes(trimmed_tokens[0], WHITESPACE);

	char	**expected_0 = (char *[]){"ls", "-l", "somedir", NULL};

	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_0, split_tokens_0, 4);

	arr_free(trimmed_tokens);
	arr_free(split_tokens);
	arr_free(split_tokens_0);
}

void	test_can_ignore_quotes() {
	char	*input = strdup("echo hello world '>' file < file2");
	char	**tokens = split_outside_quotes(input, WHITESPACE);
	char	**expected = (char *[]){"echo", "hello", "world", "'>'", "file", "<", "file2", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 8);
	free(input);
	arr_free(tokens);
}

void	test_split_no_leaks() {
	char	*input = strdup(">tmp_out | echo 1");
	char	**tokens = split_outside_quotes(input, "|");
	char	**expected = (char *[]){">tmp_out ", " echo 1", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 3);
	free(input);
	arr_free(tokens);
}

void	test_split_spaceshit() {
	char	*input = strdup("cat << delim | cat");
	char	**tokens = split_outside_quotes(input, "|");
	char	**expected = (char *[]){"cat << delim ", " cat", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 3);
	free(input);
	char	***expected_two = (char **[]){(char *[])
				{"cat", "<<", "delim", NULL},
		(char *[]){"cat", NULL}, NULL};
	char	**tokens_two_first = split_outside_quotes(tokens[0], WHITESPACE);
	char	**tokens_two_second = split_outside_quotes(tokens[1], WHITESPACE);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[0], tokens_two_first, 4);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[1], tokens_two_second, 2);
	arr_free(tokens);
	arr_free(tokens_two_first);
	arr_free(tokens_two_second);
}

void	test_split_spaceshit_two() {
	char	*input = strdup("cat | cat | ls");
	char	**tokens = split_outside_quotes(input, "|");
	char	**expected = (char *[]){"cat ", " cat ", " ls", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 4);
	free(input);
	char	**tokens_two_first;
	char	**tokens_two_second;
	char	**tokens_two_third;
	char	***expected_two = (char **[]){(char *[])
				{"cat", NULL},
		(char *[]){"cat", NULL},
		(char *[]){"ls", NULL}, NULL};
	tokens_two_first = split_outside_quotes(tokens[0], WHITESPACE);
	tokens_two_second = split_outside_quotes(tokens[1], WHITESPACE);
	tokens_two_third= split_outside_quotes(tokens[2], WHITESPACE);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[0], tokens_two_first, 2);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[1], tokens_two_second, 2);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[2], tokens_two_third, 2);
	arr_free(tokens_two_first);
	arr_free(tokens_two_second);
	arr_free(tokens_two_third);
	arr_free(tokens);
}

void	test_split_mult_pipes() {
	char	*input = strdup("echo hello    || echo world | cat");
	char	**tokens = split_outside_quotes(input, "|");
	char	**expected = (char *[]){"echo hello    ", " echo world ", " cat", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, tokens, 4);
	free(input);
	char	**tokens_two_first;
	char	**tokens_two_second;
	char	**tokens_two_third;
	char	***expected_two = (char **[]){(char *[])
				{"echo", "hello", NULL},
		(char *[]){"echo", "world", NULL},
		(char *[]){"cat", NULL}, NULL};
	tokens_two_first = split_outside_quotes(tokens[0], WHITESPACE);
	tokens_two_second = split_outside_quotes(tokens[1], WHITESPACE);
	tokens_two_third= split_outside_quotes(tokens[2], WHITESPACE);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[0], tokens_two_first, 2);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[1], tokens_two_second, 2);
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_two[2], tokens_two_third, 2);
	arr_free(tokens_two_first);
	arr_free(tokens_two_second);
	arr_free(tokens_two_third);
	arr_free(tokens);
}
