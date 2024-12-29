#include "unity.h"

#include "tokens.h"
#include "struct.h"
#include <stdlib.h>

#include "support_tokens.c"
#include "support_commands.c"
#include "support_msh.c"

void	test_prep_token(void)
{
	t_shell	*shell;

	// input like "ls -l somedir | cat -e | wc -l";
	shell = init_shell((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL});
	char	*line = ft_strdup(" ls -l $somedir ' ' | cat -e | wc -l");
	char	*trimmed_line = ft_strtrim(line, WHITESPACE);
	free(line);
	TEST_ASSERT_NOT_NULL(trimmed_line);

	// add the split tokens as tokens
	shell->token = (t_token *)get_tokens(trimmed_line);

	TEST_ASSERT_EQUAL_STRING("ls -l $somedir ' ' ", shell->token[0].split_pipes);
	TEST_ASSERT_EQUAL_STRING(" cat -e ", shell->token[1].split_pipes);
	TEST_ASSERT_EQUAL_STRING(" wc -l", shell->token[2].split_pipes);

	destroy_all_tokens(shell);
	arr_free(shell->env);
	free_null(&trimmed_line);
	// arr_free(shell->split_pipes);
	free_null(&shell);
}
