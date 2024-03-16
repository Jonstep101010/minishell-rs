#include "unity.h"

#include "tokens.h"
#include "struct.h"
#include <stdlib.h>

#include "support_tokens.c"
#include "support_commands.c"
#include "redirection_utils.c"
#include "support_msh.c"

#ifndef PRINT_TEST
# define PRINT_TEST 0
#endif

// input can be ls > file1 > file2 >file3
// handle spaces: next cmd_args index
// in this case the first two redirs are assigned to REDIR_REMOVED and the word following it will be of type REDIR_WORD, it will be assigned the redir (if any)

static t_arg	*support_redir_simple_one() {
	t_arg	*arg = init_cmdargs(9);
	arg[0].elem = strdup("ls");
	arg[1].elem = strdup(">");
	arg[2].elem = strdup("file1");// this is a REDIR_WORD of redir OUTPUT_REDIR
	arg[3].elem = strdup("<<");
	arg[4].elem = strdup("file2");// this is a REDIR_WORD of redir HEREDOC
	arg[5].elem = strdup(">>file3");// this is a REDIR of redir APPEND
	arg[6].elem = strdup(">file4");// this is a REDIR of redir OUTPUT_REDIR
	arg[7].elem = strdup("<file5");// this is a REDIR of redir INPUT_REDIR
	arg[8].elem = strdup("<<file6");// this is a REDIR of redir HEREDOC
	return (arg);
}

static t_arg	*support_redir_expected_simple_one() {
	t_arg	*arg = init_cmdargs(9);
	arg[0].elem = strdup("ls");
	arg[0].type = STRING;
	arg[0].redir = NO_REDIR;
	arg[1].elem = strdup("file1");// this is a REDIR_WORD of redir OUTPUT_REDIR
	arg[1].type = REDIR_WORD;
	arg[1].redir = OUTPUT_REDIR;
	arg[2].elem = strdup("file2");// this is a REDIR_WORD of redir HEREDOC
	arg[2].type = REDIR_WORD;
	arg[2].redir = HEREDOC;
	arg[3].elem = strdup("file3");// this is a REDIR of redir APPEND
	arg[3].type = REDIR;
	arg[3].redir = APPEND;
	arg[4].elem = strdup("file4");// this is a REDIR of redir OUTPUT_REDIR
	arg[4].type = REDIR;
	arg[4].redir = OUTPUT_REDIR;
	arg[5].elem = strdup("file5");// this is a REDIR of redir INPUT_REDIR
	arg[5].type = REDIR;
	arg[5].redir = INPUT_REDIR;
	arg[6].elem = strdup("file6");// this is a REDIR of redir HEREDOC
	arg[6].type = REDIR;
	arg[6].redir = HEREDOC;
	return (arg);
}

// enum e_redir redir[7] = {NO_REDIR, OUTPUT_REDIR, HEREDOC, APPEND, OUTPUT_REDIR, INPUT_REDIR, HEREDOC};

// possibly parse this to turn redir_word into redir @follow-up
// enum e_arg type[7] = {STRING, REDIR_WORD, REDIR_WORD, REDIR, REDIR, REDIR, REDIR};

void	test_support_can_free() {
	t_arg	*arg = support_redir_simple_one();
	// rm_prefix_redir_word(arg);
	for (int i = 0; i < 9; i++)
	{
		if (arg[i].elem)
			free(arg[i].elem);
	}
	free(arg);
}

void	test_assert_can_distinguish() {
	t_arg	*arg = support_redir_simple_one();
	TEST_ASSERT(check_redirections(arg));
	enum e_redir redir[9] = {NO_REDIR, OUTPUT_REDIR, NO_REDIR, HEREDOC, NO_REDIR, APPEND, OUTPUT_REDIR, INPUT_REDIR, HEREDOC};
	for (int i = 0; i < 9; i++)
	{
		TEST_ASSERT_EQUAL_INT(redir[i], arg[i].redir);
		free(arg[i].elem);
	}
	free(arg);
}

// @audit no more than 2 redirs per elem/word (other than quoted strings)
// @follow-up check that there is no redirection in the REDIR_WORD (lexer), and that it is non null

void	test_assert_trim() {
	t_arg	*arg = support_redir_simple_one();
	TEST_ASSERT(check_redirections(arg));
	parse_redir_types(arg);
	enum e_arg type[9] = {STRING, REDIR_REMOVED, REDIR_WORD, REDIR_REMOVED, REDIR_WORD, REDIR, REDIR, REDIR, REDIR};
	for (int i = 0; i < 9; i++)
		TEST_ASSERT_EQUAL_INT(type[i], arg[i].type);
	enum e_redir redir[9] = {NO_REDIR, OUTPUT_REDIR, OUTPUT_REDIR, HEREDOC, HEREDOC, APPEND, OUTPUT_REDIR, INPUT_REDIR, HEREDOC};
	for (int i = 0; i < 9; i++)
	{
		TEST_ASSERT_EQUAL_INT(redir[i], arg[i].redir);
	}
	char	**expected = (char *[]){"ls", "file1", "file2", "file3", "file4", "file5", "file6", NULL};
	// we remove all elements marked as REDIR_REMOVED
	rm_prefix_redir_word(arg);
	enum e_arg type_rm[7] = {STRING, REDIR_WORD, REDIR_WORD, REDIR, REDIR, REDIR, REDIR};
	enum e_redir redir_rm[7] = {NO_REDIR, OUTPUT_REDIR, HEREDOC, APPEND, OUTPUT_REDIR, INPUT_REDIR, HEREDOC};
	for (int i = 0; i < 7; i++)
	{
		TEST_ASSERT_EQUAL_STRING(expected[i], arg[i].elem);
		TEST_ASSERT_EQUAL_INT(type_rm[i], arg[i].type);
		TEST_ASSERT_EQUAL_INT(redir_rm[i], arg[i].redir);
		#if PRINT_TEST
		if (arg[i].type == REDIR_WORD)
			fprintf(stderr, "redir_word ");
		if (arg[i].type == REDIR)
			fprintf(stderr, "redir ");
		if (arg[i].type == REDIR_REMOVED)
			TEST_FAIL_MESSAGE("redir should have been removed");
		if (arg[i].redir == OUTPUT_REDIR)
			fprintf(stderr, "output: %s\n", arg[i].elem);
		else if (arg[i].redir == INPUT_REDIR)
			fprintf(stderr, "input: %s\n", arg[i].elem);
		else if (arg[i].redir == APPEND)
			fprintf(stderr, "append: %s\n", arg[i].elem);
		else if (arg[i].redir == HEREDOC)
			fprintf(stderr, "heredoc: %s\n", arg[i].elem);
		#endif
	}
	TEST_ASSERT_NULL(arg[7].elem);
	t_arg	*expected_arg = support_redir_expected_simple_one();
	for (int i = 0; i < 7; i++)
	{
		TEST_ASSERT_EQUAL_INT(expected_arg[i].redir, arg[i].redir);
		TEST_ASSERT_EQUAL_INT(expected_arg[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_STRING(expected_arg[i].elem, arg[i].elem);
		free(arg[i].elem);
		free(expected_arg[i].elem);
	}
	free(arg);
	free(expected_arg);
}

static t_arg	*support_redir_expected_simplify_one() {
	t_arg	*arg = init_cmdargs(7);
	arg[0].elem = strdup("ls");
	arg[0].type = STRING;
	arg[0].redir = NO_REDIR;
	arg[1].elem = strdup("file1");// this is a REDIR_WORD of redir OUTPUT_REDIR
	arg[1].type = REDIR;
	arg[1].redir = OUTPUT_REDIR;
	arg[2].elem = strdup("file2");// this is a REDIR_WORD of redir HEREDOC
	arg[2].type = REDIR;
	arg[2].redir = HEREDOC;
	arg[3].elem = strdup("file3");// this is a REDIR of redir APPEND
	arg[3].type = REDIR;
	arg[3].redir = APPEND;
	arg[4].elem = strdup("file4");// this is a REDIR of redir OUTPUT_REDIR
	arg[4].type = REDIR;
	arg[4].redir = OUTPUT_REDIR;
	arg[5].elem = strdup("file5");// this is a REDIR of redir INPUT_REDIR
	arg[5].type = REDIR;
	arg[5].redir = INPUT_REDIR;
	arg[6].elem = strdup("file6");// this is a REDIR of redir HEREDOC
	arg[6].type = REDIR;
	arg[6].redir = HEREDOC;
	return (arg);
}

void	test_assert_simplify() {
	t_arg	*arg = support_redir_simple_one();
	TEST_ASSERT(check_redirections(arg));
	parse_redir_types(arg);
	rm_prefix_redir_word(arg);
	t_arg	*expected = support_redir_expected_simplify_one();
	for (int i = 0; i < 7; i++)
	{
		if (arg[i].type == REDIR_WORD)// integration into functions @follow-up
			arg[i].type = REDIR;
		TEST_ASSERT_EQUAL_STRING(expected[i].elem, arg[i].elem);
		TEST_ASSERT_EQUAL_INT(expected[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_INT(expected[i].redir, arg[i].redir);
		free(arg[i].elem);
		free(expected[i].elem);
	}
	free(arg);
	free(expected);
}

void	test_assert_simplify_two() {
	t_shell	*shell = support_clean_env((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	shell->token = tokenize(shell, "ls <cmd1 > cmd2 >> cmd3");
	t_arg	*arg = shell->token->cmd_args;
	t_arg	*expected = init_cmdargs(5);
	expected[0].elem = strdup("ls");
	expected[0].type = STRING;
	expected[0].redir = NO_REDIR;
	expected[1].elem = strdup("cmd1");
	expected[1].type = REDIR;
	expected[1].redir = INPUT_REDIR;
	expected[2].elem = strdup("cmd2");
	expected[2].type = REDIR;
	expected[2].redir = OUTPUT_REDIR;
	expected[3].elem = strdup("cmd3");
	expected[3].type = REDIR;
	expected[3].redir = APPEND;
	for (int i = 0; i < 5; i++)
	{
		if (arg[i].type == REDIR_WORD || arg[i].type == REDIR_REMOVED)// integration into functions @follow-up
			TEST_FAIL();
		TEST_ASSERT_EQUAL_STRING(expected[i].elem, arg[i].elem);
		TEST_ASSERT_EQUAL_INT(expected[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_INT(expected[i].redir, arg[i].redir);
		free(expected[i].elem);
	}
	free(expected);
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free(shell);
}
