#include "libft.h"
#include "unity.h"
#include "utils.h"
#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>

#include "split_outside_quotes.c"
#include "arr_utils.c"
#include "find_key.c"
#include "print_arr_sep.c"
#include "occurs.c"
#include "expand_variables.c"

#ifndef TOKENS_H
# define TEST_TOKENS_H
// @audit-info testing version of tokens and struct
typedef struct s_token	t_token;

enum e_arg
{
	STRING,
	PIPE,
	EXPANSION,
};

enum e_quote
{
	SINGLE,
	DOUBLE,
	NONE,
};

typedef struct s_arg
{
	char	*elem;
	enum	e_arg	type;
	enum	e_quote	quote;
}	t_arg;

struct s_token
{
	t_arg	*cmd_args;// keep attributes in execution (i.e. redirs), cmd_args[0] is the first token/command (not pipe)
	char	*split_pipes;
	// char	**args;
	char	**tmp_arr;
	char	**command;// for execution (each token has the command)
	// size_t	status;// for usage with the pipes?
	// char	*bin;// for finding path/to/bin?
	int		(*func)(t_token *);
};

#include <termios.h>
typedef struct s_shell
{
	char	**split_tokens;
	char	**split_pipes;
	int		exit_status;
	char	**owned_envp;
	char	*line;
	char	*expanded_line;
	char	*tmp;
	char	**tmp_arr;
	// char	**command;// not sure if we need this
	t_token	*token;
	struct termios	p_termios;
}	t_shell;
#endif


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
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#include "build_tokens.c"

void	test_token_struct(void)
{
	t_shell	*shell;
	shell = (t_shell *) malloc(sizeof(t_shell));
	shell->line = "ls -l somedir | cat -e | wc -l";
	shell->owned_envp = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	shell->token = init_token(3);// should have space for 3 tokens (shell->line)
	TEST_ASSERT_NOT_NULL(shell->token);
	char	**expected = (char *[]){"ls -l somedir ", " cat -e ", " wc -l", NULL};
	char	**split_tokens = split_outside_quotes(shell->line, '|');
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, split_tokens, 4);

	// do string trim on all spaces inside the split tokens
	char	**trimmed_tokens = arr_trim(split_tokens, " ");
	char	**expected_trimmed = (char *[]){"ls -l somedir", "cat -e", "wc -l", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_trimmed, trimmed_tokens, 4);

	char	**split_tokens_0 = split_outside_quotes(trimmed_tokens[0], ' ');
	char	**split_tokens_1 = split_outside_quotes(trimmed_tokens[1], ' ');
	char	**split_tokens_2 = split_outside_quotes(trimmed_tokens[2], ' ');

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
	shell = (t_shell *) malloc(sizeof(t_shell));
	shell->line = "ls -l somedir | cat -e | wc -l";
	shell->owned_envp = (char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", NULL};
	char	**split_pipe_tokens = split_outside_quotes(shell->line, '|');
	shell->split_tokens = arr_trim(split_pipe_tokens, " ");// trim the spaces
	add_pipe_split_as_tokens(shell->split_tokens, shell);
	TEST_ASSERT_NOT_NULL(shell->token);
	TEST_ASSERT_EQUAL_STRING("ls -l somedir", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);
	free(shell->token);
	arr_free(shell->split_tokens);
	arr_free(split_pipe_tokens);
	free(shell);
}

void	test_convert_string_array_to_tokens(void)
{
	t_shell	*shell;

	// input like "ls -l somedir | cat -e | wc -l";
	shell = (t_shell *) malloc(sizeof(t_shell));
	char	*input = ft_strdup(" ls -l $somedir ' ' | cat -e | wc -l");

	shell->split_pipes = split_outside_quotes(input, '|');
	free(input);

	// trim beforehand
	shell->split_tokens = arr_trim(shell->split_pipes, " ");
	arr_free(shell->split_pipes);
	char	**expected = arr_dup((const char **)((char *[]){"ls -l $somedir ' '", "cat -e", "wc -l", NULL}));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, shell->split_tokens, 4);
	shell->owned_envp = arr_dup((const char **)((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL}));

	// add the split tokens as tokens
	add_pipe_split_as_tokens(shell->split_tokens, shell);
	TEST_ASSERT_NOT_NULL(shell->token->split_pipes);

	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' '", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);

	convert_split_token_string_array_to_tokens(shell);
	TEST_ASSERT_NOT_NULL(shell->token->cmd_args);

	// first column should be the token, second column should be the index of the command/args
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING("' '", shell->token[0].cmd_args[3].elem);
	printf("from test_convert_string_array_to_tokens: $%s\n", shell->token[0].cmd_args[3].elem);
	// TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd[0].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_INT(SINGLE, shell->token[0].cmd_args[3].quote);
	TEST_ASSERT_EQUAL_INT(STRING, shell->token[0].cmd_args[2].type);
	TEST_ASSERT_EQUAL_INT(NONE, shell->token[0].cmd_args[2].quote);

	// free all heap memory

	// free first array
	free(shell->token[0].cmd_args[0].elem);
	free(shell->token[0].cmd_args[1].elem);
	free(shell->token[0].cmd_args[2].elem);
	free(shell->token[0].cmd_args[3].elem);

	// free second array
	free(shell->token[1].cmd_args[0].elem);
	free(shell->token[1].cmd_args[1].elem);

	// free third array
	free(shell->token[2].cmd_args[0].elem);
	free(shell->token[2].cmd_args[1].elem);

	free(shell->token[0].cmd_args);
	free(shell->token[1].cmd_args);
	free(shell->token[2].cmd_args);
	free(shell->token);
	arr_free(shell->split_tokens);
	arr_free(shell->owned_envp);
	arr_free(expected);
	free(shell);
}

t_shell	*support_test_tokens_as_input_execution(void)
{
	t_shell	*shell;

	// input like "ls -l somedir | cat -e | wc -l";
	shell = (t_shell *) malloc(sizeof(t_shell));
	char	*input = ft_strdup(" ls -l $somedir ' ' | cat -e | wc -l");

	shell->split_pipes = split_outside_quotes(input, '|');
	free(input);

	// trim beforehand
	shell->split_tokens = arr_trim(shell->split_pipes, " ");
	arr_free(shell->split_pipes);
	char	**expected = arr_dup((const char **)((char *[]){"ls -l $somedir ' '", "cat -e", "wc -l", NULL}));
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, shell->split_tokens, 4);
	shell->owned_envp = arr_dup((const char **)((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL}));

	// add the split tokens as tokens
	add_pipe_split_as_tokens(shell->split_tokens, shell);
	// TEST_ASSERT_EQUAL_size_t(3, shell->token_len);
	TEST_ASSERT_NOT_NULL(shell->token->split_pipes);

	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' '", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING("cat -e", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING("wc -l", shell->token[2].split_pipes);

	convert_split_token_string_array_to_tokens(shell);
	TEST_ASSERT_NOT_NULL(shell->token->cmd_args);

	// first column should be the token, second column should be the index of the command/args
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING("' '", shell->token[0].cmd_args[3].elem);

	// TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd[0].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	TEST_ASSERT_EQUAL_INT(SINGLE, shell->token[0].cmd_args[3].quote);
	TEST_ASSERT_EQUAL_INT(STRING, shell->token[0].cmd_args[2].type);
	TEST_ASSERT_EQUAL_INT(NONE, shell->token[0].cmd_args[2].quote);

	arr_free(expected);
	return (shell);
}

void	function_that_needs_pointer_to_token_instance(t_token *token)
{
	char	*tmp;
	tmp = token->cmd_args[2].elem;
	token->cmd_args[2].elem = token->cmd_args[0].elem;
	token->cmd_args[0].elem = tmp;
}

void	test_token_is_modifiable() {
	t_shell	*shell = support_test_tokens_as_input_execution();
	function_that_needs_pointer_to_token_instance(&(shell->token[0]));
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[2].elem);
	// free first array
	free(shell->token[0].cmd_args[0].elem);
	free(shell->token[0].cmd_args[1].elem);
	free(shell->token[0].cmd_args[2].elem);
	free(shell->token[0].cmd_args[3].elem);

	// free second array
	free(shell->token[1].cmd_args[0].elem);
	free(shell->token[1].cmd_args[1].elem);

	// free third array
	free(shell->token[2].cmd_args[0].elem);
	free(shell->token[2].cmd_args[1].elem);

	free(shell->token[0].cmd_args);
	free(shell->token[1].cmd_args);
	free(shell->token[2].cmd_args);
	free(shell->token);
	arr_free(shell->split_tokens);
	arr_free(shell->owned_envp);
	free(shell);
}

#include "destroy_tokens.c"
void	test_token_struct_freeing() {
	t_shell	*shell = support_test_tokens_as_input_execution();
	destroy_all_tokens(shell);
	arr_free(shell->split_tokens);
	arr_free(shell->owned_envp);
	free(shell);
}

void	test_destroy_single_token() {
	t_shell	*shell = support_test_tokens_as_input_execution();
	t_token	*token = &(shell->token[1]);
	remove_token(shell, token);
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_NULL(shell->token[2].cmd_args);
	destroy_all_tokens(shell);
	arr_free(shell->split_tokens);
	arr_free(shell->owned_envp);
	free(shell);
}

void	test_destroy_null() {
	t_shell	*shell = support_test_tokens_as_input_execution();
	t_token	*token = NULL;
	remove_token(shell, token);
	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("you", shell->token[0].cmd_args[2].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);
	destroy_all_tokens(shell);
	arr_free(shell->split_tokens);
	arr_free(shell->owned_envp);
	free(shell);
}

t_shell	*support_test_tokens_input(char *line, char **envp)
{
	t_shell	*shell;

	// input like "ls -l somedir | cat -e | wc -l";
	shell = (t_shell *) malloc(sizeof(t_shell));
	char	*input = ft_strdup(line);

	shell->split_pipes = split_outside_quotes(input, '|');
	free(input);

	// trim beforehand
	shell->split_tokens = arr_trim(shell->split_pipes, " ");
	arr_free(shell->split_pipes);
	shell->owned_envp = arr_dup((const char **)envp);
	TEST_ASSERT_NOT_NULL(shell->owned_envp);

	// add the split tokens as tokens
	add_pipe_split_as_tokens(shell->split_tokens, shell);
	TEST_ASSERT_NOT_NULL(shell->token->split_pipes);

	convert_split_token_string_array_to_tokens(shell);
	TEST_ASSERT_NOT_NULL(shell->token->cmd_args);
	return (shell);
}

void	test_recursive_expansion() {
	t_shell	*shell = support_test_tokens_input("ls -l $somedir ' ' | cat -e | wc -l", (char **)((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));

	TEST_ASSERT_NOT_NULL(shell->token[0].split_pipes);
	// we want recursive expansion
	TEST_ASSERT_EQUAL_STRING("mypath$", shell->token[0].cmd_args[2].elem);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("' '", shell->token[0].cmd_args[3].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	destroy_all_tokens(shell);
	arr_free(shell->owned_envp);
	arr_free(shell->split_tokens);
	free(shell);
}

#include "build_command.c"

void	test_convert_tokens_to_string_array(void) {
	t_shell	*shell = support_test_tokens_input("ls -l $somedir ' ' | cat -e | wc -l", (char **)((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));

	TEST_ASSERT_NOT_NULL(shell->token[0].split_pipes);
	// we want recursive expansion
	TEST_ASSERT_EQUAL_STRING("mypath$", shell->token[0].cmd_args[2].elem);

	TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("' '", shell->token[0].cmd_args[3].elem);
	TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);
	TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	convert_tokens_to_string_array(shell->token);
	TEST_ASSERT_NOT_NULL(shell->token->command);

	char	**expected1 = (char *[]){"ls", "-l", "mypath$", "' '", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected1, shell->token[0].command, 4);

	char	**expected2 = (char *[]){"cat", "-e", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected2, shell->token[1].command, 2);

	char	**expected3 = (char *[]){"wc", "-l", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected3, shell->token[2].command, 2);

	arr_free(shell->token[0].command);
	arr_free(shell->token[1].command);
	arr_free(shell->token[2].command);
	destroy_all_tokens(shell);
	arr_free(shell->owned_envp);
	arr_free(shell->split_tokens);
	free(shell);
}

void	test_convert_single_command_no_pipes(void) {
	t_shell	*shell = support_test_tokens_input("$somedir", (char **)((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));

	TEST_ASSERT_NOT_NULL(shell->token[0].split_pipes);
	// we want recursive expansion
	TEST_ASSERT_EQUAL_STRING("mypath$", shell->token[0].cmd_args[0].elem);

	// TEST_ASSERT_EQUAL_STRING("ls", shell->token[0].cmd_args[0].elem);
	// TEST_ASSERT_EQUAL_STRING("-l", shell->token[0].cmd_args[1].elem);
	// TEST_ASSERT_EQUAL_STRING("' '", shell->token[0].cmd_args[3].elem);
	// TEST_ASSERT_EQUAL_STRING("cat", shell->token[1].cmd_args[0].elem);
	// TEST_ASSERT_EQUAL_STRING("-e", shell->token[1].cmd_args[1].elem);
	// TEST_ASSERT_EQUAL_STRING("wc", shell->token[2].cmd_args[0].elem);
	// TEST_ASSERT_EQUAL_STRING("-l", shell->token[2].cmd_args[1].elem);

	convert_tokens_to_string_array(shell->token);
	TEST_ASSERT_NOT_NULL(shell->token->command);

	char	**expected = (char *[]){"mypath$", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, shell->token[0].command, 1);
	arr_free(shell->token[0].command);
	arr_free(shell->token[1].command);
	destroy_all_tokens(shell);
	arr_free(shell->owned_envp);
	arr_free(shell->split_tokens);
	free(shell);
}
