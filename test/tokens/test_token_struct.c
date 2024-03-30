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

#include "support_commands.c"
// environment
#include "support_msh.c"

// get module support
#include "support_tokens.c"

char	**arr_trim(char **arr, char const *set)
{
	size_t	i;
	size_t	len;
	char	**ret;

	if (!arr)
		return (NULL);
	if (!set)
		return (arr);
	i = 0;
	len = arr_len(arr);
	ret = malloc(sizeof(char *) * (len + 1));
	if (!ret)
		return (NULL);
	while (arr[i])
	{
		ret[i] = ft_strtrim(arr[i], set);
		i++;
	}
	ret[i] = NULL;
	return (ret);
}

void	test_token_struct(void)
{
	t_shell	*shell;
	shell = (t_shell *) calloc(1, sizeof(t_shell));
	char	*line = "ls \n-l\r \tsomedir | cat -e | wc -l";
	shell->env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	shell->token = init_token(3);// should have space for 3 tokens (shell->line)
	TEST_ASSERT_NOT_NULL(shell->token);
	char	**expected = (char *[]){"ls \n-l\r \tsomedir ", " cat -e ", " wc -l", NULL};
	char	**split_tokens = split_outside_quotes(line, "|");
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
	char	*trimmed_line = "ls -l somedir | cat -e | wc -l";
	shell->env = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	tokenize(shell, trimmed_line);
	TEST_ASSERT_NOT_NULL(shell->token);
	// TEST_ASSERT_EQUAL_STRING("ls -l somedir ", shell->token[0].split_pipes);
	// TEST_ASSERT_EQUAL_STRING(" cat -e ", shell->token[1].split_pipes);
	// TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("somedir", shell->token[0].cmd_args[2].elem);

	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);
	destroy_all_tokens(shell);
	free(shell->token);
	free(shell);
}

void	test_destroy_null() {
	t_shell	*shell = support_test_tokens((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL});
	tokenize(shell, "ls -l you | wc -l");
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_NULL(shell->token[2].cmd_args);
	cleanup_support_test_token(shell);
}

void	test_recursive_expansion() {
	t_shell	*shell = support_test_tokens(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));
	tokenize(shell, "ls -l $somedir ' ' | cat -e | wc -l");
	// TEST_ASSERT_NOT_NULL(shell->token[0].split_pipes);
	// we want recursive expansion
	TEST_ASSERT_EQUAL_STRING("$otherdir", shell->token[0].cmd_args[2].elem);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING(" ", shell->token[0].cmd_args[3].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	cleanup_support_test_token(shell);
}

void	test_export_env() {
	t_shell	*shell = support_clean_env(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", "true=true", NULL}));

	export_env(shell, ft_strdup("true=false"));
	char	**expected = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", "true=false", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, shell->env, 6);
	TEST_ASSERT_NOT_NULL(tokenize(shell, "unset true"));
	// TEST_ASSERT_EQUAL_STRING("unset true", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("unset", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("true", shell->token[0].cmd_args[1].elem);
	builtin_unset(shell, &shell->token[0]);
	char	**expected_rm = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_rm, shell->env, 6);
	TEST_ASSERT_NULL(get_env(shell->env, "true"));
	cleanup_support_test_token(shell);
}

void	test_no_leaks() {
	t_shell	*shell = support_test_tokens(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));
	shell->token = tokenize(shell, ">tmp_out | echo 1");
	TEST_ASSERT_NOT_NULL(shell->token);
	TEST_ASSERT_EQUAL_STRING("tmp_out", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_INT(OUTPUT_REDIR, shell->token[0].cmd_args[0].redir);
	TEST_ASSERT_EQUAL_STRING("echo", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("1", shell->token[1].cmd_args[1].elem);
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free(shell);
}

void	test_no_leaks_two() {
	t_shell	*shell = support_test_tokens(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));
	shell->token = tokenize(shell, ">tmp_out | echo 1");
	TEST_ASSERT_NOT_NULL(shell->token);
	TEST_ASSERT_EQUAL_STRING("tmp_out", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_INT(OUTPUT_REDIR, shell->token[0].cmd_args[0].redir);
	TEST_ASSERT_EQUAL_STRING("echo", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("1", shell->token[1].cmd_args[1].elem);
	char	**cmd_arr_null = get_cmd_arr_token(&shell->token[0]);
	TEST_ASSERT_NULL(cmd_arr_null);
	char	**cmd_arr = get_cmd_arr_token(&shell->token[1]);
	TEST_ASSERT_NOT_NULL(cmd_arr);
	TEST_ASSERT_NULL(cmd_arr[2]);
	TEST_ASSERT_EQUAL_STRING("echo", cmd_arr[0]);
	TEST_ASSERT_EQUAL_STRING("1", cmd_arr[1]);
	destroy_all_tokens(shell);
	arr_free(shell->env);
	free(shell);
	arr_free(cmd_arr);
}

void	test_corrupted_input() {
	t_shell	*shell = support_test_tokens(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));
	shell->token = tokenize(shell, "cat | cat | ls");
	TEST_ASSERT_NOT_NULL(shell->token);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_INT(NO_REDIR, shell->token[0].cmd_args[0].redir);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[2].cmd_args[0].elem);
	cleanup_support_test_token(shell);
}

void	test_corrupted_input_two() {
	t_shell	*shell = support_test_tokens(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));
	shell->token = tokenize(shell, "<tmp_out cat");
	TEST_ASSERT_NOT_NULL(shell->token);
	TEST_ASSERT_EQUAL_STRING("tmp_out", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_INT(INPUT_REDIR, shell->token[0].cmd_args[0].redir);
	TEST_ASSERT_EQUAL_INT(NO_REDIR, shell->token[0].cmd_args[1].redir);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[0].cmd_args[1].elem);
	cleanup_support_test_token(shell);
}

void test_can_create_mult_noleaks() {
	t_shell	*shell = support_clean_env((char *[]){"", "PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL});
	shell->token = tokenize(shell, "cat | cat | ls");
	TEST_ASSERT_NOT_NULL(shell->token);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[2].cmd_args[0].elem);
	cleanup_support_test_token(shell);
}
