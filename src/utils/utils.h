#ifndef UTILS_H
# define UTILS_H

int	str_cchr(const char *s, char c);

char	*occurs(const char *big, const char *little);
char	*occurs_exclusively(const char *expected, const char *actual);

#include <stddef.h>
void	print_arr_sep(char **arr, char sep_open, char sep_close);
char	**append_str_arr(char **arr, const char *s);
void	rm_str_arr(char **arr, const char *s);

#include <stdbool.h>
bool	*bool_arr_zeroing(size_t len);
void	range_ignore(const char *s, bool *ignore, unsigned char c);
#endif