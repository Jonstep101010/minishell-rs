#include "unity.h"
#include "support_lib.c"
#include "interpret_quotes.c"

void	test_nested_one() {
	// "'"'"'test'"'"'"
	// "'"     '"'      test   '"'    "'"
	// '"test"'
	char	*input = "\"'\"'\"'test'\"'\"'\"";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "'\"test\"'";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

void	test_nested_one_two() {
	// "'"test"'" -> "'" test "'"
	char	*input = "\"'\"test\"'\"";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected2 = "'test'";
	TEST_ASSERT_EQUAL_STRING(expected2, line);
	free(line);
}

void	test_nested_two() {
	// echo "'" '"' test "'" '"'
	// ' " test ' "
	char	*input = "echo \"'\" '\"' test \"'\" '\"'";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo ' \" test ' \"";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

// /* cases:

// echo 'hello tehre 'hello inside single' "jesus""'
// -> quotes not closed (error)

// echo 'hello tehre  'hello inside single' jesus'
// -> hello tehre hello inside single jesus

// */

// // export TESTONE="'"test"'" -> "'" test "'"
// // -> TESTONE='test'

// // "'$USER'" - only in doublequotes expansion (nested single work)
// // '$USER' - does not expand

// single quoted runs from one to the next (anything in between is a character)
// no way to use single quotes to denote single quotes, e.g. '''
void	test_nested_three() {
	// echo ''hello'' -> hello
	char	*input = "echo ''test''";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo test";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

void	test_nested_three_two() {
	// echo ""hello"" -> hello
	char	*input = "echo \"\"test\"\"";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo test";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

// in case of minishell, only expansion is to be considered (no escapes)
// echo "'hello'" -> 'hello' (sq just characters)
// echo '"hello"' -> "hello"
void	test_nested_four() {
	char	*input = "\"'hello'\"";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "'hello'";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

void	test_nested_five() {
	char	*input = "'\"hello\"'";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "\"hello\"";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}
// // echo "$PATH" -> value of path
// // echo ""'$PATH'"" -> prints $PATH bc singles & ignored ""

// // echo "'$PATH'" -> value of path inside singlequotes
// // echo "$PATH" -> value of path
// // echo "$'PATH'" -> prints $'PATH'

// // find next matching, (if next, ignore entirely)
// // otherwise treat what is inside as literal/expand

// echo "hello tehre 'hello inside single' "jesus""
// -> hello tehre 'hello inside single' jesus
void	test_inside_one() {
	char	*input = "echo \"hello tehre 'hello inside single' \"jesus\"\"";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo hello tehre 'hello inside single' jesus";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

void	test_inside_two() {
	// echo 'hello tehre 'hello inside single' "jesus"'
	// -> hello tehre hello inside single "jesus"
	char	*input = "echo 'hello tehre 'hello inside single' \"jesus\"'";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo hello tehre hello inside single \"jesus\"";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

void	test_inside_two_two() {
	// echo 'hello tehre 'hello inside single' jesus""'
	// -> hello tehre hello inside single jesus""
	char	*input = "echo 'hello tehre 'hello inside single' jesus\"\"'";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo hello tehre hello inside single jesus\"\"";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

void	test_inside_three() {
	// echo 'hello tehre  'hello inside single' jesus'''
	// -> hello tehre hello inside single jesus
	char	*input = "echo 'hello tehre 'hello inside single' jesus'''";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo hello tehre hello inside single jesus";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}

// should not remove repeating doublequotes inside singles
void	test_inside_four() {
	// echo 'hello tehre 'hello inside single' ""jesus""'
	// -> hello tehre hello inside single ""jesus""
	char 	*input = "echo 'hello tehre 'hello inside single' \"\"jesus\"\"'";
	char	*line = do_quote_bs(input, &(int){0});
	char	*expected = "echo hello tehre hello inside single \"\"jesus\"\"";
	TEST_ASSERT_EQUAL_STRING(expected, line);
	free(line);
}
