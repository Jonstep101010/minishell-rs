#include "unity.h"
// make sure we can tokenize a line that is split by pipes
// then each token has a command and arguments, which are stored in an array of structs that contain the command/args and their attributes

// an argument can be quoted or not, but if quoted, it should only contain the
// one type of quotes and it must be delimited (i.e. no mixing of quotes/quoted and unquoted)

// ls -l somedir | cat -e | wc -l
// ls -l "somedir" | cat -e | wc -l
// ls -l 'somedir' | cat -e | wc -l
// ls -l "$somedir" | cat -e | wc -l
// echo "$something"significant | cat -e | wc -l
// echo 'something'significant | cat -e | wc -l -> the single quoted part must be stored as a single argument?
#include "tokens.h"
#include "struct.h"
#include <stdlib.h>

#include "split_outside_quotes.c"
#include "support_lib.c"
#include "utils.h"
#include "arr_utils.c"
#include "occurs.c"
#include "token_utils.c"
#include "support_commands.c"
#include "destroy_tokens.c"
#include "error.c"
#include "build_command.c"
#include "init.c"


#include "build_tokens.c"
#include "expander.c"
#include "expand_variables.c"
#include "interpret_quotes.c"

#define TEST_SOURCE_FILE(build_tokens)
void	test_token_struct(void)
{
	t_shell	*shell;
	shell = (t_shell *) calloc(1, sizeof(t_shell));
	shell->line = "ls \n-l\r \tsomedir | cat -e | wc -l";
	shell->owned_envp = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	shell->token = init_token(3);// should have space for 3 tokens (shell->line)
	TEST_ASSERT_NOT_NULL(shell->token);
	char	**expected = (char *[]){"ls \n-l\r \tsomedir ", " cat -e ", " wc -l", NULL};
	char	**split_tokens = split_outside_quotes(shell->line, "|");
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, split_tokens, 4);

	// do string trim on all spaces inside the split tokens
	char	**trimmed_tokens = arr_trim(split_tokens, WHITESPACE);
	char	**expected_trimmed = (char *[]){"ls \n-l\r \tsomedir", "cat -e", "wc -l", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_trimmed, trimmed_tokens, 4);

	char	**split_tokens_0 = split_outside_quotes(trimmed_tokens[0], WHITESPACE);
	char	**split_tokens_1 = split_outside_quotes(trimmed_tokens[1], WHITESPACE);
	char	**split_tokens_2 = split_outside_quotes(trimmed_tokens[2], WHITESPACE);

	TEST_ASSERT_NOT_NULL(split_tokens);
	TEST_ASSERT_NOT_NULL(split_tokens_0);
	TEST_ASSERT_NOT_NULL(split_tokens_1);
	TEST_ASSERT_NOT_NULL(split_tokens_2);

	// make space and assign
	shell->token[0].cmd_args = init_cmdargs(3);
	shell->token[0].cmd_args[0].elem = split_tokens_0[0];
	shell->token[0].cmd_args[1].elem = split_tokens_0[1];
	shell->token[0].cmd_args[2].elem = split_tokens_0[2];

	// make space and assign
	shell->token[1].cmd_args = init_cmdargs(2);
	shell->token[1].cmd_args[0].elem = split_tokens_1[0];
	shell->token[1].cmd_args[1].elem = split_tokens_1[1];

	shell->token[2].cmd_args = init_cmdargs(2);// "wc" "-l"
	shell->token[2].cmd_args[0].elem = split_tokens_2[0];
	shell->token[2].cmd_args[1].elem = split_tokens_2[1];

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("somedir", shell->token[0].cmd_args[2].elem);

	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	free(shell->token[0].cmd_args);
	free(shell->token[1].cmd_args);
	free(shell->token[2].cmd_args);
	arr_free(split_tokens);
	arr_free(trimmed_tokens);
	arr_free(split_tokens_0);
	arr_free(split_tokens_1);
	arr_free(split_tokens_2);
	free(shell->token);
	free(shell);
}

void	test_add_string_array_as_tokens()
{
	t_shell	*shell;
	shell = (t_shell *) calloc(1, sizeof(t_shell));
	shell->exit_status = 0;
	shell->trimmed_line = "ls -l somedir | cat -e | wc -l";
	shell->owned_envp = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	add_pipes_as_tokens(shell);
	TEST_ASSERT_NOT_NULL(shell->token);
	TEST_ASSERT_EQUAL_STRING("ls -l somedir", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);
	free(shell->token);
	arr_free(shell->split_pipes);
	free(shell);
}

t_shell	*support_test_tokens(const char *line, char *envp[])
{
	t_shell	*shell;
	shell = init_shell((const char **)envp);
	shell->exit_status = 0;
	shell->line = ft_strdup(line);
	shell->trimmed_line = ft_strtrim(shell->line, WHITESPACE);
	TEST_ASSERT_NOT_NULL(shell->trimmed_line);
	free(shell->line);
	return (shell);
}

void	cleanup_support_test_token(t_shell *shell)
{
	destroy_all_tokens(shell);
	arr_free(shell->owned_envp);
	free_null(&shell->trimmed_line);
	arr_free(shell->split_pipes);
	free_null(&shell);
}

void test_support_test_tokens(void)
{
	t_shell	*shell = support_test_tokens(" ls -l $somedir ' ' | cat -e | wc -l", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL});

	add_pipes_as_tokens(shell);

	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' '", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);

	convert_split_token_string_array_to_tokens(shell);
	TEST_ASSERT_NOT_NULL(shell->token->cmd_args);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING(" ", shell->token[0].cmd_args[3].elem);

	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_INT(SINGLE, shell->token[0].cmd_args[3].quote);
	TEST_ASSERT_EQUAL_INT(STRING, shell->token[0].cmd_args[2].type);
	TEST_ASSERT_EQUAL_INT(NONE, shell->token[0].cmd_args[2].quote);

	cleanup_support_test_token(shell);
}

void	test_destroy_null() {
	t_shell	*shell = support_test_tokens("ls -l you | wc -l", (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	remove_token(shell, NULL);
	add_pipes_as_tokens(shell);
	convert_split_token_string_array_to_tokens(shell);
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_NULL(shell->token[2].cmd_args);
	cleanup_support_test_token(shell);
}

void	test_recursive_expansion() {
	t_shell	*shell = support_test_tokens("ls -l $somedir ' ' | cat -e | wc -l", ((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));
	add_pipes_as_tokens(shell);
	convert_split_token_string_array_to_tokens(shell);
	TEST_ASSERT_NOT_NULL(shell->token[0].split_pipes);
	// we want recursive expansion
	TEST_ASSERT_EQUAL_STRING("mypath$", shell->token[0].cmd_args[2].elem);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING(" ", shell->token[0].cmd_args[3].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	cleanup_support_test_token(shell);
}
