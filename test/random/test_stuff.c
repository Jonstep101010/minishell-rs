#include "unity.h"

int	has_static_add(int n, const int *start_nullable)
{
	static int	i = 0;
	if (start_nullable)
		i = *start_nullable;
	i += n;
	return (i);
}

void	test_has_static_add() {
	int	test = -10;
	TEST_ASSERT_EQUAL(5, has_static_add(5, NULL));
	TEST_ASSERT_EQUAL(5, has_static_add(0, NULL));
	TEST_ASSERT_EQUAL(10, has_static_add(5, NULL));
	TEST_ASSERT_EQUAL(-5, has_static_add(5, &test));
}
