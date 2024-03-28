#include "checks_basic.c"
#include "lexer_support.c"
#include "check_quotes.c"
#include "check_pipes.c"
#include "bool_array.c"
#include "unity.h"
#include <stdbool.h>
#include "support_lib.c"
#include "error.c"

void test_redir_pipes_mix()
{
	TEST_ASSERT(lexer_checks_basic(">|")->lexer != 0);
	TEST_ASSERT(lexer_checks_basic("> infile | cat")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("cat << delim | > tmp_out")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("<|")->lexer != 0);
	TEST_ASSERT(lexer_checks_basic("<infile | cat")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("> |")->lexer != 0);
	TEST_ASSERT(lexer_checks_basic("< |")->lexer != 0);
}

// no trailing redir/pipe
// no leading pipe, no redir without quotes/file
void	test_redirs() {
	TEST_ASSERT(lexer_checks_basic("echo > outfile")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("> outfile")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("< outfile")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("< < <")->lexer != 0);
	TEST_ASSERT(lexer_checks_basic("> > >")->lexer != 0);
	TEST_ASSERT(lexer_checks_basic("> > >")->lexer != 0);
	TEST_ASSERT(lexer_checks_basic("> tmpfile > midfile >")->lexer != 0);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("cat << delim | > outfile")->lexer);
}

void test_works_with_newlines()
{
	TEST_ASSERT(lexer_checks_basic("\n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n \n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n	\n \n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n\n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n\n\n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\n\n\n\n")->lexer == 0);
	TEST_ASSERT(lexer_checks_basic("\n\n\n\n\n\\n\n\n\n\n")->lexer == 0);
}

void test_works_with_quotes() {

	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo \"This is a test\"")->lexer);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo \"Hello, World!\"")->lexer);
	TEST_ASSERT(0 != lexer_checks_basic("echo Hello, World!\"")->lexer);
	TEST_ASSERT(0 != lexer_checks_basic("echo \"Hello, World!")->lexer);
	TEST_ASSERT(0 != lexer_checks_basic("echo \'Hello, World!")->lexer);
	TEST_ASSERT(0 != lexer_checks_basic("echo 'Hello, World!")->lexer);
	TEST_ASSERT(0 != lexer_checks_basic("'\"'")->lexer);
	TEST_ASSERT(0 != lexer_checks_basic("echo \"'Hello, World!\"")->lexer);
	TEST_ASSERT(0 != lexer_checks_basic("\"'\"")->lexer);
}

// void test_trailing_leading_quote() {
// 	TEST_ASSERT(lexer_checks_basic("\"Hello, World!\"")->lexer == 0);
// 	TEST_ASSERT(lexer_checks_basic("\"Hello, World!") == LEXER_DOUBLE_QUOTE);
// 	TEST_ASSERT(lexer_checks_basic("\"Hello, World!") == LEXER_DOUBLE_QUOTE);
// 	TEST_ASSERT(lexer_checks_basic("echo 'Hello, World!") == LEXER_SINGLE_QUOTE);
// 	TEST_ASSERT(lexer_checks_basic("echo Hello, World!'") == LEXER_SINGLE_QUOTE);
// 	TEST_ASSERT(lexer_checks_basic("echo \"Hello, World!'")->lexer != 0);
// 	TEST_ASSERT(lexer_checks_basic("echo Hello, World!\"") == LEXER_DOUBLE_QUOTE);
// 	TEST_ASSERT(lexer_checks_basic("echo \"Hello, World!\"")->lexer == 0);
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("'echo'"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("if (x > 5) { printf(\"x is greater than 5\"); }"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("awk '{count++} END {print count}'"));
// 	TEST_ASSERT_EQUAL(0, 		lexer_checks_basic("int x = 5; printf(The value of x is %d, x);"));
// 	TEST_ASSERT_EQUAL(0, 		lexer_checks_basic("for (int i = 0; i < 5; i++) { printf(\"%d\n\", i); }"));
// 	TEST_ASSERT_EQUAL(0, 		lexer_checks_basic("while (i < 10) { i++; }"));
// }

// test () {} [] each and unbalanced aquivalents
// void	test_parenthesis() {
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo (Hello, World!)"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo {Hello, World!}"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo Hello, World!}"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo Hello, World!)"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo (Hello, World!"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("if (x > 5 {{ printf(\"x is greater than 5\"})); }"));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("while (i < 10) { i++; "));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("while (i < 10) i++;} "));
// 	TEST_ASSERT_EQUAL(0, lexer_checks_basic("echo {Hello, World!"));
// }

void	test_redir_quotes() {
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("cat <\"./test_files/infile\" | echo hi")->lexer);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("cat << \"$USER\"")->lexer);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("cat << \"$US\"E\"R\"")->lexer);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic(">> \"$USER'$USER'\"")->lexer);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("\"$USER'$USER'\" | echo")->lexer);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("\"$USER'$USER'\" | \"echo\"")->lexer);
	TEST_ASSERT_EQUAL(0, lexer_checks_basic("$USER'$USER' | \"echo\"")->lexer);
}

/*
flag redir, word set to 1
ignore to true
*/
