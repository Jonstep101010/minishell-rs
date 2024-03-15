#include "unity.h"

#include "tokens.h"
#include "struct.h"
#include <stdlib.h>

#include "support_tokens.c"
#include "support_commands.c"
#include "support_msh.c"

void	rm_prefix_redir_word(t_arg *arg)
{
	size_t i;
	size_t len;

	if (!arg)
		return;

	i = 0;
	len = 0;
	while (arg[len].elem)
		len++;
	while (arg[i].elem)
	{
		if (arg[i].type == REDIR_REMOVED)
		{
			free(arg[i].elem);
			while (i < len)
			{
				arg[i] = arg[i + 1];
				i++;
			}
			return (rm_prefix_redir_word(arg));
		}
		i++;
	}
}

// input can be ls > file1 > file2 >file3
// handle spaces: next cmd_args index
// in this case the first two redirs are assigned to REDIR_REMOVED and the word following it will be of type REDIR_WORD, it will be assigned the redir (if any)

static t_arg	*support_redir_simple_one() {
	t_arg	*arg = init_cmdargs(9);
	arg[0].elem = strdup("ls");
	arg[1].elem = strdup(">");
	arg[2].elem = strdup("file1");
	arg[3].elem = strdup("<<");
	arg[4].elem = strdup("file2");
	arg[5].elem = strdup(">>file3");
	arg[6].elem = strdup(">file4");
	arg[7].elem = strdup("<file5");
	arg[8].elem = strdup("<<file6");
	return (arg);
}

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

void	test_assert_trim() {
	t_arg	*arg = support_redir_simple_one();
	TEST_ASSERT(check_redirections(arg));

	int i = 0;
	char	*tmp;
	while (arg[i].elem)
	{
		if (arg[i].type != REDIR_WORD
			&& (arg[i].redir == INPUT_REDIR || arg[i].redir == OUTPUT_REDIR))
		{
			if (arg[i].elem[1])
			{
				tmp = strdup(&arg[i].elem[1]);
				free(arg[i].elem);
				arg[i].elem = tmp;
				arg[i].type = REDIR;
			}
			else
				arg[i].type = REDIR_REMOVED;
		}
		else if (arg[i].type != REDIR_WORD
			&& (arg[i].redir == APPEND || arg[i].redir == HEREDOC))
		{
			if (arg[i].elem[2])
			{
				tmp = strdup(&arg[i].elem[2]);
				free(arg[i].elem);
				arg[i].elem = tmp;
				arg[i].type = REDIR;
			}
			else
				arg[i].type = REDIR_REMOVED;
		}
		// check something like this in lexer
		//if (arg[i + 1].elem && arg[i + 1].redir == NO_REDIR)
		// @audit no more than 2 redirs per elem/word (other than quoted strings)
		if (arg[i].type == REDIR_REMOVED)
		{
			arg[i + 1].type = REDIR_WORD;
			arg[i + 1].redir = arg[i].redir;
		}
		fprintf(stderr, "elem: %s\n", arg[i].elem);
		if (arg[i].type == REDIR_WORD)
			fprintf(stderr, "redir: %d\n", arg[i].redir);
		// @follow-up check that there is no redirection in the REDIR_WORD (lexer, and that it is non null)
		i++;
	}
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
	for (int i = 0; i < 7; i++)
	{
		TEST_ASSERT_EQUAL_STRING(expected[i], arg[i].elem);
		fprintf(stderr, "left: %s\n", arg[i].elem);
	}
	TEST_ASSERT_NULL(arg[7].elem);
	for (int i = 0; i < 7; i++)
		free(arg[i].elem);
	free(arg);
}
