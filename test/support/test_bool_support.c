#include "support_bool.h"

void test_support_expected() {
	char	s[] = "0110011100011111000";
	// (_""he'y'ayo"the"lay)
	bool	expected[20] = {false, true, true, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, false, false};
	bool	*actual = support_expected(s);
	if (!actual)
		TEST_FAIL();
	for (int i = 0; i <= (int)strlen(s); i++)
	{
		fprintf(stderr, "i: %d, actual[i]: %d, expected[i]: %d\n", i, actual[i], expected[i]);
		TEST_ASSERT_EQUAL(expected[i], actual[i]);
	}
	free(actual);
}

void test_support_expected2() {
	char s[] = "101010101010101110";
	bool expected[20] = {true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, false, false};
	bool *actual = support_expected(s);
	if (!actual)
		TEST_FAIL();
	for (int i = 0; i <= (int)strlen(s); i++){
		fprintf(stderr, "i: %d, actual[i]: %d, expected[i]: %d\n", i, actual[i], expected[i]);
		TEST_ASSERT_EQUAL(expected[i], actual[i]);
	}
	free(actual);
}


void test_can_work_test_struct()
{
	struct s_test *test = returns_test_struct("\"'\"", "1110");
	for (int i = 0; i <= test->slen; i++) {
		TEST_ASSERT_EQUAL(test->expected[i], test->actual[i]);
	}
	cleanup_test_struct(test);
}

void test_can_work_2()
{
	struct s_test *test = returns_test_struct("this is my input \"ignore\" 't' 'this' \"is\" 'a' \"test\" 'string'", "00000000000000000111111110111011111101111011101111110111111110");
	for (int i = 0; i <= test->slen; i++) {
		TEST_ASSERT_EQUAL(test->expected[i], test->actual[i]);
	}
	cleanup_test_struct(test);
}
