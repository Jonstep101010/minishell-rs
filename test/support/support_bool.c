#include "../src/bool_array.c"
#include "support_bool.h"
#include <stdlib.h>

void support_print(const char *s, const bool *arr) {
	for (int i = 0; i <= (int)strlen(s); i++){
		fprintf(stderr, "i: %d, arr[i]: %d, s[i]: %c\n", i, arr[i], s[i]);
	}
	for (int i = 0; i <= (int)strlen(s); i++){
		if (arr[i] == true)
			fprintf(stderr, "%d", arr[i]);
		else
			fprintf(stderr, "%d", arr[i]);
	}
	fprintf(stderr,"\n");
}

// provide a wrapper for bool_arr_zeroing
void	support_bool_arr_zeroing(const char *s, bool **ignore, size_t len)
{
	(void)s;
	*ignore = bool_arr_zeroing(len);
}

// convert char array to bool array
bool *support_expected(const char *s) {

	bool 	*arr = NULL;
	int		len = (int)strlen(s);

	support_bool_arr_zeroing(s, &arr, len);
	if (!arr)
		return (NULL);
	for (int i = 0; i <= len; i++) {
		if (s[i] == '1')
			arr[i] = true;
		else
			arr[i] = false;
	}
	return (arr);
}

/* boilerplate to prep tests */
t_test *returns_test_struct(const char *s, const char *expected) {
	struct s_test *test = calloc(1, sizeof(struct s_test));
	if (!test)
		return (NULL);
	test->actual = NULL;
	test->slen = (int)strlen(s);

	support_bool_arr_zeroing(s, &(test->actual), test->slen);
	if (!test->actual)
		return (NULL);
	range_ignore(s, test->actual, '\'');
	range_ignore(s, test->actual, '\"');
	test->expected = support_expected(expected);
	return (test);
}

void cleanup_test_struct(struct s_test *test) {
	free(test->expected);
	free(test->actual);
	free(test);
}
