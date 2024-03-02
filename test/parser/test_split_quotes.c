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

int	has_static_add(int n, const int *start_nullable)
{
	static int	i = 0;
	if (start_nullable)
		i = *start_nullable;
	i += n;
	return (i);
}

void	test_has_static_add() {
	int	test = -10;
	TEST_ASSERT_EQUAL(5, has_static_add(5, NULL));
	TEST_ASSERT_EQUAL(5, has_static_add(0, NULL));
	TEST_ASSERT_EQUAL(10, has_static_add(5, NULL));
	TEST_ASSERT_EQUAL(-5, has_static_add(5, &test));
}


static int is_in_set(char c, const char *set)
{
	while (*set)
	{
		if (c == *set)
			return (1);
		set++;
	}
	return (0);
}

char	**append_str_arr_free(char **arr, char *s);
// do not touch unless tested changes -> this leaks like a *****
static char	**split_iterator(const char *to_split,
		char *trimmed, const char *set)
{
	t_splitter	*split;
	char		**ret;
	split = (t_splitter *)ft_calloc(1, sizeof(t_splitter));
	while (trimmed[split->i] && split->start < ft_strlen(trimmed))
	{
		if (split->quote == 0 &&
			(trimmed[split->i] == '\'' || trimmed[split->i] == '"'))
				split->quote = trimmed[split->i];
		else if (split->quote != 0 && trimmed[split->i] == split->quote)
			split->quote = 0;
		else if (split->quote == 0 && is_in_set(trimmed[split->i], set))
		{
			while (is_in_set(trimmed[split->i + 1], set))
			{
				split->i++;
				if (split->i >= ft_strlen(trimmed))
					break;
			}
			if (split->token_end < split->start)
				split->token_end = split->start - 1;
			split->tmp = ft_substr(trimmed, split->start,
					split->token_end - split->start + 1);
			if (!split->tmp)
				return (arr_free(split->arr), NULL);
			split->ret = append_str_arr((const char **)split->arr, split->tmp);
			free(split->tmp);
			arr_free(split->arr);
			if (!split->ret)
				return (arr_free(split->ret), NULL);
			split->arr = split->ret;
			split->start = split->i + 1;
		}
		else
			split->token_end = split->i;
		split->i++;
	}
	free(trimmed);
	ret = append_str_arr_free(split->ret, ft_substr(to_split, split->start, split->i - split->start));
	return (free(split), ret);
}

char	**split_outside_quotes(const char *to_split, const char *set)
{
	char		**ret;
	char		*trimmed;

	if (!to_split)
		return (NULL);
	while (is_in_set(*to_split, set))
		to_split++;
	trimmed = ft_strtrim(to_split, set);
	if (!trimmed)
		return (NULL);
	ret = split_iterator(to_split, trimmed, set);
	return (ret);
}

#include "free_strjoin.c"
#include "expander.c"
#include "expand_variables.c"
#include "find_key.c"
#include "arr_utils.c"
#include "occurs.c"

#include "strtrim_outside_quotes.c"

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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
	TEST_ASSERT_EQUAL(arr_len((const char **)tokens), arr_len((const char **)expected));
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
