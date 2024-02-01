#include "unity.h"
char	**split_outside_quotes(const char *to_split, char c);
#include "support_parser.h"
void	test_expander() {
	char	*line = "echo $PAGER";
	char	*envp[] = {"TEST_SOME=false", "PAGER=true", NULL};
	TEST_ASSERT_NOT_NULL(line);
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo true";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*expanded_line = expand_variables(line, (const char **)envp);
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

	char	*expanded_line = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(expanded_line);
	TEST_ASSERT_EQUAL_STRING(expected_ret, expanded_line);
	free(expanded_line);
}

void	test_expander_two() {
	char	*line = "echo $PAGER | echo \"$TEST\" | echo $TEST | echo $PAGER";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo true | echo \"false\" | echo false | echo true";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}


// // echo hello$PAGER -> echo helloVAL
// // echo "$PAGER" -> echo VAL
// // echo $PAGER -> echo VAL
// // echo $'PAGER'S -> echo PAGERS (remove $)
// // echo '$PAGER' -> echo $PAGER
// // echo $"PAGER"S -> echo PAGERS
// // echo $PAGER_S -> echo VAL

void	test_expander_ignore_in_singlequotes() {
	char	*line = "echo '$PAGER'";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo '$PAGER'";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
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

	char	*actual = expand_variables(line, (const char **)envp);
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

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_singlequotes_key() {
	char	*line = "echo $'TEST'";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo  'TEST'";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_doublequotes_key() {
	char	*line = "echo $\"TEST\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo  \"TEST\"";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_sq_key() {
	char	*line = "echo $'TEST $TEST'";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo  'TEST $TEST'";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

void	test_expander_ignore_in_doublequotes_key_two() {
	char	*line = "echo \"$PAGER\"";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};

	char	*expected_ret = "echo \"true\"";

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	printf("%s\n", actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	free(actual);
}

// echo $'PAGER'S -> echo  PAGERS (remove $ and replace with space)
void	test_expander_followed_sq() {
	char	*line = "echo $'PAGER'hello";
	char	*envp[] = {"PAGER=true", "TEST=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo  'PAGER'hello";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
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

	char	*expected_ret = "echo  'PAGER'false";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// should this case even be implemented?
// void	test_error_invalid_name() {
// 	char	*line = "echo $'PA?GER'$test";
// 	char	*envp[] = {"PA?GER=true", "test=false", NULL};
// 	TEST_ASSERT_NOT_NULL(envp);

// 	char	*expected_ret = "echo  'PA?GER'false";
// 	TEST_ASSERT_NOT_NULL(expected_ret);

// 	char	*actual = expand_variables(line, (const char **)envp);
// 	TEST_ASSERT_NOT_NULL(actual);
// 	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
// 	printf("%s\n", actual);
// 	free(actual);
// }

void	test_replace_key_not_found_name() {
	char	*line = "echo $hello";
	char	*envp[] = {"PAGER=true", "test=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo ";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}

// weird stuff with invalid names
void	test_invalid_key_found_name() {
	char	*line = "echo $h?echo";
	char	*envp[] = {"PAGER=true", "test=false", NULL};
	TEST_ASSERT_NOT_NULL(envp);

	char	*expected_ret = "echo ?echo";
	TEST_ASSERT_NOT_NULL(expected_ret);

	char	*actual = expand_variables(line, (const char **)envp);
	TEST_ASSERT_NOT_NULL(actual);
	TEST_ASSERT_EQUAL_STRING(expected_ret, actual);
	printf("%s\n", actual);
	free(actual);
}
