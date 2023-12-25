#ifndef SUPPORT_BOOL_H_
# define SUPPORT_BOOL_H_

# include <stdbool.h>
typedef struct s_test {
	bool	*expected;
	bool	*actual;
	int		slen;
}	t_test;

# include <stddef.h>
void	support_bool_arr_zeroing(const char *s, bool **ignore, size_t len);
bool	*support_expected(const char *s);
t_test *returns_test_struct(const char *s, const char *expected);
void	cleanup_test_struct(struct s_test *test);
void	support_print(const char *s, const bool *arr);

#include "unity.h"
#include <string.h>
#include <stdlib.h>
#endif