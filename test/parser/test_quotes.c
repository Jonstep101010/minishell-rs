#include "unity.h"
#include "support_parser.h"

t_shell	*setup_shell(char *line) {
	t_shell *shell = (t_shell *) calloc(sizeof(t_shell), 1);
	shell->line = strdup(line);
	if (!shell || !shell->line)
		TEST_FAIL();
	if (lexer(shell->line) != LEXER_SUCCESS)
		TEST_FAIL_MESSAGE("lexer failed!");
	return (shell);
}

void	clean_shell(t_shell *shell) {
	if (shell->line)
		free_null(shell->line);
	if (shell->command)
		arr_free(shell->command);
	free_null(shell);
}

void	debug_print_arr(t_shell *this)
{
	for (int i = 0; this->command[i]; i++)
	{
		fprintf(stderr, "string in arr:%s\n", this->command[i]);
	}
}

void	test_nested_one() {
	// "'"'"'test'"'"'"
	// "'"     '"'      test   '"'    "'"
	// '"test"'
	t_shell *this = setup_shell("\"'\"'\"'test'\"'\"'\"");
	int	quote = 0;
	this->line = do_quote_bs(this->line, &quote);
	char	*expected = "'\"test\"'";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
	// "'"test"'" -> "'" test "'"
	quote = 0;
	this = setup_shell("\"'\"test\"'\"");
	this->line = do_quote_bs(this->line, &quote);
	char	*expected2 = "'test'";
	TEST_ASSERT_EQUAL_STRING(expected2, this->line);
	clean_shell(this);
}

void	test_nested_two() {
	// echo "'" '"' test "'" '"'
	// ' " test ' "
	int	quote = 0;
	t_shell	*this = setup_shell("echo \"'\" '\"' test \"'\" '\"'");
	this->line = do_quote_bs(this->line, &quote);
	char	*expected = "echo ' \" test ' \"";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
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
	t_shell	*this = setup_shell("echo ''test''");
	int	quote = 0;
	char	*expected = "echo test";
	this->line = do_quote_bs(this->line, &quote);
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
	// echo ""hello"" -> hello
	this = setup_shell("echo \"\"test\"\"");
	quote = 0;
	this->line = do_quote_bs(this->line, &quote);
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
}

// in case of minishell, only expansion is to be considered (no escapes)
// echo "'hello'" -> 'hello' (sq just characters)
// echo '"hello"' -> "hello"
void	test_nested_four() {
	t_shell	*this = setup_shell("\"'hello'\"");
	int	quote = 0;
	this->line = do_quote_bs(this->line, &quote);
	char	*expected = "'hello'";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
	quote = 0;
	this = setup_shell("'\"hello\"'");
	this->line = do_quote_bs(this->line, &quote);
	expected = "\"hello\"";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
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
	int	quote = 0;
	t_shell	*this = setup_shell("echo \"hello tehre 'hello inside single' \"jesus\"\"");
	this->line = do_quote_bs(this->line, &quote);
	char	*expected = "echo hello tehre 'hello inside single' jesus";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
}

void	test_inside_two() {
	// echo 'hello tehre 'hello inside single' "jesus"'
	// -> hello tehre hello inside single "jesus"
	int	quote = 0;
	t_shell	*this = setup_shell("echo 'hello tehre 'hello inside single' \"jesus\"'");
	this->line = do_quote_bs(this->line, &quote);
	char	*expected = "echo hello tehre hello inside single \"jesus\"";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
	// echo 'hello tehre 'hello inside single' jesus""'
	// -> hello tehre hello inside single jesus""
	quote = 0;
	this = setup_shell("echo 'hello tehre 'hello inside single' jesus\"\"'");
	this->line = do_quote_bs(this->line, &quote);
	expected = "echo hello tehre hello inside single jesus\"\"";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
}

void	test_inside_three() {
	// echo 'hello tehre  'hello inside single' jesus'''
	// -> hello tehre hello inside single jesus
	int	quote = 0;
	t_shell *this = setup_shell("echo 'hello tehre 'hello inside single' jesus'''");
	this->line = do_quote_bs(this->line, &quote);
	char	*expected = "echo hello tehre hello inside single jesus";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
}

// should not remove repeating doublequotes inside singles
void	test_inside_four() {
	// echo 'hello tehre 'hello inside single' ""jesus""'
	// -> hello tehre hello inside single ""jesus""
	int	quote = 0;
	t_shell *this = setup_shell("echo 'hello tehre 'hello inside single' \"\"jesus\"\"'");
	this->line = do_quote_bs(this->line, &quote);
	char	*expected = "echo hello tehre hello inside single \"\"jesus\"\"";
	TEST_ASSERT_EQUAL_STRING(expected, this->line);
	clean_shell(this);
}
