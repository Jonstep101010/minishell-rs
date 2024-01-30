#include "unity.h"
// #include "expand_variables.c"

#include "support_parser.h"
void	test_expander() {
	char	*line = strdup("echo $PAGER | echo $TEST_SOME");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST_SOME=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo true | echo false");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	// printf("%s\n", actual);
}

void	test_expander_two() {
	char	*line = strdup("echo $PAGER | echo \"$TEST\" | echo $TEST | echo $PAGER");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo true | echo \"false\" | echo false | echo true");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}


// echo hello$PAGER -> echo helloVAL
// echo "$PAGER" -> echo VAL
// echo $PAGER -> echo VAL
// echo $'PAGER'S -> echo PAGERS (remove $)
// echo '$PAGER' -> echo $PAGER
// echo $"PAGER"S -> echo PAGERS
// echo $PAGER_S -> echo VAL

void	test_expander_ignore_in_singlequotes() {
	char	*line = strdup("echo '$PAGER'");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo '$PAGER'");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

void	test_expander_followed() {
	char	*line = strdup("echo $PAGER$TEST");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo truefalse");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

void	test_expander_followed_dq() {
	char	*line = strdup("echo \"$PAGER\"$TEST");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo \"true\"false");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

void	test_expander_ignore_in_singlequotes_key() {
	char	*line = strdup("echo $'TEST'");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo  'TEST'");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

void	test_expander_ignore_in_doublequotes_key() {
	char	*line = strdup("echo $\"TEST\"");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo  \"TEST\"");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

void	test_expander_ignore_in_doublequotes() {
	char	*line = strdup("echo \"$PAGER\"");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo \"true\"");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

// echo $'PAGER'S -> echo  PAGERS (remove $ and replace with space)
void	test_expander_followed_sq() {
	char	*line = strdup("echo $'PAGER'hello");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo  'PAGER'hello");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

// echo $'PAGER'S -> echo  PAGERS (remove $ and replace with space)
void	test_expander_followed_sq_var() {
	char	*line = strdup("echo $'PAGER'$TEST");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PAGER=true"), "TEST=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo  'PAGER'false");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}

void	test_error_invalid_name() {
	char	*line = strdup("echo $'PA?GER'$test");
	char	**envp = NULL;
	envp = append_str_arr(append_str_arr(envp, "PA?GER=true"), "test=false");
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = strdup("echo  'PA?GER'false");
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
}
