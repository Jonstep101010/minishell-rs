#include "unity.h"

#include "tokens.h"
#include "struct.h"
#include <stdlib.h>

#include "support_tokens.c"
#include "support_commands.c"
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

void	test_support_can_free() {
	t_arg	*arg = support_redir_simple_one();
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

void	test_assert_trim() {
	t_arg	*arg = support_redir_simple_one();
	TEST_ASSERT(check_redirections(arg));
	parse_redir_types(arg);
	enum e_arg type[9] = {STRING, REDIR_REMOVED, STRING, REDIR_REMOVED, STRING, REDIR, REDIR, REDIR, REDIR};
	for (int i = 0; i < 9; i++)
		TEST_ASSERT_EQUAL_INT(type[i], arg[i].type);
	enum e_redir redir[9] = {NO_REDIR, OUTPUT_REDIR, NO_REDIR, HEREDOC, NO_REDIR, APPEND, OUTPUT_REDIR, INPUT_REDIR, HEREDOC};
	for (int i = 0; i < 9; i++)
		TEST_ASSERT_EQUAL_INT(redir[i], arg[i].redir);
	char	**expected = (char *[]){"ls", "file1", "file2", "file3", "file4", "file5", "file6", NULL};
	// we remove all elements marked as REDIR_REMOVED
	rm_prefix_redir_word(arg);
	enum e_arg type_rm[7] = {STRING, REDIR, REDIR, REDIR, REDIR, REDIR, REDIR};
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
	t_arg	expected_arg[7] = {
		{.elem = strdup("ls"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup("file1"), .type = REDIR, .redir = OUTPUT_REDIR},
		{.elem = strdup("file2"), .type = REDIR, .redir = HEREDOC},
		{.elem = strdup("file3"), .type = REDIR, .redir = APPEND},
		{.elem = strdup("file4"), .type = REDIR, .redir = OUTPUT_REDIR},
		{.elem = strdup("file5"), .type = REDIR, .redir = INPUT_REDIR},
		{.elem = strdup("file6"), .type = REDIR, .redir = HEREDOC},
	};
	for (int i = 0; i < 7; i++)
	{
		TEST_ASSERT_EQUAL_INT(expected_arg[i].redir, arg[i].redir);
		TEST_ASSERT_EQUAL_INT(expected_arg[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_STRING(expected_arg[i].elem, arg[i].elem);
		free(arg[i].elem);
		free(expected_arg[i].elem);
	}
	free(arg);
}

void	test_assert_simplify() {
	t_arg	*arg = support_redir_simple_one();
	TEST_ASSERT(check_redirections(arg));
	parse_redir_types(arg);
	rm_prefix_redir_word(arg);
	t_arg	expected[7] = {
		{.elem = strdup("ls"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup("file1"), .type = REDIR, .redir = OUTPUT_REDIR},
		{.elem = strdup("file2"), .type = REDIR, .redir = HEREDOC},
		{.elem = strdup("file3"), .type = REDIR, .redir = APPEND},
		{.elem = strdup("file4"), .type = REDIR, .redir = OUTPUT_REDIR},
		{.elem = strdup("file5"), .type = REDIR, .redir = INPUT_REDIR},
		{.elem = strdup("file6"), .type = REDIR, .redir = HEREDOC},
	};
	for (int i = 0; i < 7; i++)
	{
		TEST_ASSERT_EQUAL_STRING(expected[i].elem, arg[i].elem);
		TEST_ASSERT_EQUAL_INT(expected[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_INT(expected[i].redir, arg[i].redir);
		free(arg[i].elem);
		free(expected[i].elem);
	}
	free(arg);
}

void	test_assert_simplify_two() {
	t_shell	*shell = support_clean_env((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	shell->token = tokenize(shell, "ls <cmd1 > cmd2 >> cmd3");
	t_arg	*arg = shell->token->cmd_args;
	t_arg	expected[5] = {
		{.elem = strdup("ls"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup("cmd1"), .type = REDIR, .redir = INPUT_REDIR},
		{.elem = strdup("cmd2"), .type = REDIR, .redir = OUTPUT_REDIR},
		{.elem = strdup("cmd3"), .type = REDIR, .redir = APPEND},
	};
	for (int i = 0; i < 5; i++)
	{
		TEST_ASSERT_EQUAL_STRING(expected[i].elem, arg[i].elem);
		TEST_ASSERT_EQUAL_INT(expected[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_INT(expected[i].redir, arg[i].redir);
		free(expected[i].elem);
	}
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free(shell);
}

void	test_assert_shell_redir_one() {
	t_shell	*shell = support_clean_env((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	shell->token = tokenize(shell, "ls <cmd1 > cmd2 >> cmd3");
	t_arg	*arg = shell->token->cmd_args;
	t_arg	expected[5] = {
		{.elem = strdup("ls"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup("cmd1"), .type = REDIR, .redir = INPUT_REDIR},
		{.elem = strdup("cmd2"), .type = REDIR, .redir = OUTPUT_REDIR},
		{.elem = strdup("cmd3"), .type = REDIR, .redir = APPEND},
	};
	TEST_ASSERT_EQUAL(true, shell->token->has_redir);
	for (int i = 0; i < 5; i++)
	{
		TEST_ASSERT_EQUAL_STRING(expected[i].elem, arg[i].elem);
		TEST_ASSERT_EQUAL_INT(expected[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_INT(expected[i].redir, arg[i].redir);
		free(expected[i].elem);
	}
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free(shell);
}

void	test_assert_shell_no_redir_one() {
	t_shell	*shell = support_clean_env((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	shell->token = tokenize(shell, "echo hello world '>' \">file\" file2");
	t_arg	*arg = shell->token->cmd_args;
	t_arg	expected[7] = {
		{.elem = strdup("echo"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup("hello"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup("world"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup(">"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup(">file"), .type = STRING, .redir = NO_REDIR},
		{.elem = strdup("file2"), .type = STRING, .redir = NO_REDIR},
	};
	TEST_ASSERT_EQUAL(false, shell->token->has_redir);
	for (int i = 0; i < 7; i++)
	{
		TEST_ASSERT_EQUAL_STRING(expected[i].elem, arg[i].elem);
		TEST_ASSERT_EQUAL_INT(expected[i].type, arg[i].type);
		TEST_ASSERT_EQUAL_INT(expected[i].redir, arg[i].redir);
		free(expected[i].elem);
	}
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free(shell);
}
