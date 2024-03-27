#include "unity.h"
#include "libutils.h"
#include "expander.c"
#include "get_env.c"
#include "get_index.c"
#include "support_lib.c"
#include "error.c"
#include "arr_utils.c"
#include "str_equal.c"
#include "destroy_tokens.c"

void	test_expander() {
	char	*line = "echo $PAGER";
	char	*envp[] = {"TEST_SOME=false", "PAGER=true", NULL};
	TEST_ASSERT_NOT_NULL(line);
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo true";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*expanded_line = expander(line, envp);
	TEST_ASSERT_NOT_NULL(expanded_line);
	TEST_ASSERT_EQUAL_STRING(expected_ret, expanded_line);
	free(expanded_line);
}

void	test_expander_mult() {
	char	*line = "echo $PAGER | echo $TEST_SOME";
	char	*envp[] = {"TEST_SOME=false", "PAGER=true", NULL};
	TEST_ASSERT_NOT_NULL(line);
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo true | echo false";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*expanded_line = expander(line, envp);
	TEST_ASSERT_EQUAL_STRING(expected_ret, expanded_line);
	free(expanded_line);
}

void	test_expander_two() {
	char	*line = "echo $PAGER | echo \"$TEST\" | echo $TEST | echo $PAGER";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo true | echo \"false\" | echo false | echo true";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// // echo hello$PAGER -> echo helloVAL
// // echo "$PAGER" -> echo VAL
// // echo $PAGER -> echo VAL
// // echo $'PAGER'S -> echo $PAGERS
// // echo '$PAGER' -> echo $PAGER
// // echo $"PAGER"S -> echo $PAGERS
void	test_expander_ignore_in_singlequotes() {
	char	*line = "echo '$PAGER'";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo '$PAGER'";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_followed() {
	char	*line = "echo $PAGER$TEST";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo truefalse";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_followed_dq() {
	char	*line = "echo \"$PAGER\"$TEST";
	char	*envp[] ={ "PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo \"true\"false";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_singlequotes_key() {
	char	*line = "$'TEST'";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "$'TEST'";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_doublequotes_key() {
	char	*line = "$\"TEST\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "$\"TEST\"";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	// TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_sq_key() {
	char	*line = "echo $'TEST $TEST'";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo $'TEST $TEST'";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_doublequotes_key_two() {
	char	*line = "echo \"$PAGER\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo \"true\"";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	printf("%s\n", actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	free(actual);
}

// // echo $'PAGER'S -> echo  PAGERS (remove $ and replace with space)
void	test_expander_followed_sq() {
	char	*line = "echo $'PAGER'hello";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo $'PAGER'hello";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo $'PAGER'S -> echo  PAGERS (remove $ and replace with space)
void	test_expander_followed_sq_var() {
	char	*line = "echo $'PAGER'$TEST";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo $'PAGER'false";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// // should this case even be implemented?
void	test_error_invalid_name() {
	char	*line = "echo $'PA?GER'$test";
	char	*envp[] = {"PA?GER=true", "test=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo $'PA?GER'false";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_replace_key_not_found_name() {
	char	*line = "$hello";
	char	*envp[] = {"PAGER=true", "test=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	free(actual);
}

void	test_key_not_found_name() {
	char	*line = "$h_echo";
	char	*envp[] = {"PAGER=true", "test=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

char	**arr_map(char **arr, void *(*f)(void *, void *), void *arg)
{
	size_t	i;
	char	**ret;

	i = 0;
	ret = (char **)ft_calloc(arr_len(arr) + 1, sizeof(char *));
	if (!ret)
		return (NULL);
	while (arr[i])
	{
		ret[i] = f(arr[i], arg);
		i++;
	}
	return (ret);
}

void	test_expansion_space_follows_non_null() {
	char	**split_tokens = arr_dup((char *[]){"ls -l $somedir ' '", "cat -e", "wc -l", NULL});

	// trim beforehand
	char	**split_tokens_trim_spaces = arr_trim(split_tokens, " ");
	char	**expected = (char *[]){"ls -l $somedir ' '", "cat -e", "wc -l", NULL};
	TEST_ASSERT_EQUAL_STRING_ARRAY(expected, split_tokens_trim_spaces, 4);
	char	**envp = arr_dup(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=you", NULL}));

	char	**expanded = arr_map(split_tokens_trim_spaces, (void *)expander, (void *)envp);

	char	**expected_expanded = (char *[]){"ls -l you ' '", "cat -e", "wc -l", NULL};

	TEST_ASSERT_EQUAL_STRING_ARRAY(expected_expanded, expanded, 4);

	arr_free(envp);
	arr_free(split_tokens_trim_spaces);
	arr_free(split_tokens);
	arr_free(expanded);
}

void	test_expansion_followed_dollarsign() {

	char	**split_tokens = arr_dup((char *[]){"ls -l $somedir", "cat -e", "wc -l", NULL});

	char	**split_tokens_trim_spaces = arr_trim(split_tokens, " ");

	char	**envp = arr_dup(((char *[]){"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL}));

	TEST_ASSERT_EQUAL_STRING("ls -l $somedir", split_tokens_trim_spaces[0]);

	char	*tmp;
	while (str_cchr(split_tokens_trim_spaces[0], '$') != 0)
	{
		tmp = expander(split_tokens_trim_spaces[0], envp);
		TEST_ASSERT_NOT_NULL(tmp);
		if (ft_strncmp(tmp, split_tokens_trim_spaces[0], ft_strlen(tmp)) == 0)
		{
			free(tmp);
			break;
		}
		free(split_tokens_trim_spaces[0]);
		split_tokens_trim_spaces[0] = tmp;
	}

	char	**expected_expanded = (char *[]){"ls -l mypath$", "cat -e", "wc -l", NULL};

	TEST_ASSERT_EQUAL_STRING(expected_expanded[0], split_tokens_trim_spaces[0]);

	arr_free(envp);
	arr_free(split_tokens);
	arr_free(split_tokens_trim_spaces);
}

void	test_input_prefix_dollar() {
	char	*line = "$$PAGER";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "$true";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_input_prefix_dollar_two() {
	char	*line = "$$PAGER";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "$true";

	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	free(actual);
}

void	test_get_exit_status() {
	char	*line = "echo $?";
	char	*envp[] = {"PAGER=true", "TEST=false", "?=0", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo 0";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_get_exit_status_other() {
	char	*line = "$?$none$some";
	char	*envp[] = {"PAGER=true", "TEST=false", "?=1", "hello=echo", "some=thing", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "1thing";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_get_exit_status_single() {
	char	*line = "$?";
	char	*envp[] = {"PAGER=true", "TEST=false", "?=1", "hello=echo", "some=thing", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "1";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_get_exit_status_mult() {
	char	*line = "$?$$?$?$$$$hello?$";
	char	*envp[] = {"PAGER=true", "TEST=false", "?=1", "hello=echo", "some=thing", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "1$11$$$$";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_nothing_to_do() {
	char	*line = "something";
	char	*envp[] = {"PAGER=true", "TEST=false", "?=1", "hello=echo", "some=thing", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "something";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_recursive_expansion() {
	char	*line = "ls -l $somedir ' ' | cat -e | wc -l";
	char	*envp[] = {"PATH=/usr/bin", "HOME=/home/user", "USER=user", "somedir=$otherdir", "otherdir=mypath$", NULL};

	char	*expected_ret = "ls -l mypath$ ' ' | cat -e | wc -l";

	char	*actual = expander(line, envp);
	// expand as many times as there are variables (excluding trailing $)
	char	*actual_second = expander(actual, envp);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual_second);

	free(actual);
	free(actual_second);
}

// echo "'$USER'" should expand to 'username' (currently '$USER')
void	test_expansion_nested_quotes() {
	char	*line = "echo \"'$PAGER'\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo \"'true'\"";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expansion_nested_quotes_not() {
	char	*line = "echo '\"$PAGER\"'";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo '\"$PAGER\"'";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo  $KEYsomething should result in empty string
void	test_bug_one() {
	char	*line = "$USERsomething";
	char	*envp[] = {"USER=username", "TEST=true", NULL};

	char	*expected_ret = "";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo "'$KEYsomething'" should result in empty inside single quotes ''
void	test_expansion_nested_quotes_two() {
	char	*line = "echo \"'$PAGERsomething'\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo \"''\"";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo $$KEY should result in $val (beginning and end)
void	test_expansion_prefix_one() {
	char	*line = "echo $$PAGER";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo $true";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo "'"'$USER'"'" -> '$USER'
void	test_expansion_nested_quotes_three() {
	char	*line = "echo \"'\"'$PAGER'\"'\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo \"'\"'$PAGER'\"'\"";

	char	*actual = expander(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo $"42$" -> $42$
void	test_expansion_prefix_two() {
	char	*line = "echo $\"42$\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo $\"42$\"";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo "$ " -> $
void	test_expansion_prefix_three() {
	char	*line = "echo \"$ \"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo \"$ \"";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo $$TEST$ -> $TEST$
void	test_expansion_prefix_trailing_leading() {
	char	*line = "echo $$TEST$";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo $false$";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo "$TEST something"  -> echo "$TEST something"
void	test_expansion_followed_other() {
	char	*line = "echo \"$TEST something\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo \"false something\"";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo $TEST something  -> $TEST something
void	test_expansion_followed_other_two() {
	char	*line = "echo $TEST something";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo false something";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo $TEST$something  -> echo false
void	test_expansion_followed_other_three() {
	char	*line = "echo $TEST$something";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo false";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// echo something  -> echo something
void	test_no_expansion() {
	char	*line = "echo something strange";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo something strange";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}
// echo something  -> echo something
void	test_no_expansion_two() {
	char	*line = "echo something $$ strange";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo something $$ strange";
	char	*actual = expander(line, envp);

	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}
