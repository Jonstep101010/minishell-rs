// @audit make these pass in new iteration

// #include "unity.h"
// #include "free_strjoin.c"
// #include "free_strjoin_utils.c"
// #include "support_lib.c"

// void	test_can_free_multiple_strings() {
// 	char	*s1 = strdup("hello");
// 	char	*s2 = strdup(" ");
// 	char	*s3 = strdup(",");
// 	char	*s4 = strdup("World");
// 	char	*s5 = strdup("!");
// 	free_n_null(5, &s1, &s2, &s3, &s4, &s5);
// }

// void	test_free_null_multiple_strings() {
// 	char	*s1 = strdup("hello");
// 	char	*s2 = strdup(" ");
// 	char	*s3 = strdup(",");
// 	char	*s4 = strdup("World");
// 	char	*s5 = strdup("!");

// 	free_n_null(5, &s1, &s2, &s3, &s4, &s5);
// 	TEST_ASSERT_NULL(s1);
// 	TEST_ASSERT_NULL(s2);
// 	TEST_ASSERT_NULL(s3);
// 	TEST_ASSERT_NULL(s4);
// 	TEST_ASSERT_NULL(s5);
// }

// void	test_join_free() {
// 	char	*s1 = strdup("hello");
// 	char	*s2 = strdup(" ");
// 	char	*s3 = strdup(",");
// 	char	*s4 = strdup("World");
// 	char	*s5 = strdup("!");
// 	char	*joined = free_strjoin(5, &s1, &s2, &s3, &s4, &s5);
// 	TEST_ASSERT_NOT_NULL(joined);
// 	TEST_ASSERT_NOT_EMPTY(joined);
// 	TEST_ASSERT_EQUAL_STRING("hello ,World!", joined);
// 	free(joined);
// 	TEST_ASSERT_NULL(s1);
// 	TEST_ASSERT_NULL(s2);
// 	TEST_ASSERT_NULL(s3);
// 	TEST_ASSERT_NULL(s4);
// 	TEST_ASSERT_NULL(s5);
// }

// void	test_null() {
// 	char	*joined = free_strjoin(0, NULL);
// 	TEST_ASSERT_NULL(joined);
// 	free_strjoin(INT_MAX, NULL);
// 	free(joined);
// 	joined = NULL;
// }

// void	test_ez() {
// 	char	*s1 = "hello";
// 	char	*s2 = strdup(" ,World!");
// 	char *joined = str_join(2, s1, s2);
// 	TEST_ASSERT_NOT_NULL(joined);
// 	TEST_ASSERT_NOT_EMPTY(joined);
// 	TEST_ASSERT_EQUAL_STRING("hello ,World!", joined);
// 	free_n(2, s2, joined);
// }

// void	test_null_strings() {
// 	char	*s1 = "hello";
// 	char	*s2 = strdup(" ,World!");
// 	char	*joined = free_second_join(s1, s2);
// 	TEST_ASSERT_EQUAL_STRING("hello ,World!", joined);
// 	free(joined);

// 	char	*tofree_s2 = strdup(" ,World!");
// 	char	*joined2 = free_second_join(NULL, tofree_s2);
// 	TEST_ASSERT_NULL(joined2);
// 	// valgrind to check!
// }
