#include "unity.h"
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

static void	bool_arr_fixture(const char *s, bool **ignore, size_t len)
{
	(void)s;
	int	i = 0;

	*ignore = (bool *) calloc(len + 1, sizeof(bool));
	while (i < (int)len)
	{
		(*ignore)[i] = false;
		i++;
	}
	(*ignore)[i] = true;
}

void test_bool_arr_can_have_a_true_value() {
	bool	*arr = NULL;
	int		i	 = 0;

	bool_arr_fixture("Hello", &arr, strlen("Hello"));
	for (i = 0; i < (int)strlen("Hello");i++)
		TEST_ASSERT_EQUAL(false, arr[i]);
	TEST_ASSERT_EQUAL(true, arr[i]);
	free(arr);
}
