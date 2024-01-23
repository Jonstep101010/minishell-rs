#ifndef UTILS_H
# define UTILS_H

int	str_cchr(const char *s, char c);

char	*occurs(const char *big, const char *little);
char	*occurs_exclusively(const char *expected, const char *actual);

#include <stddef.h>
size_t	null_arr_len(char **arr);
#endif