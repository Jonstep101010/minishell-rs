#ifndef UTILS_H
# define UTILS_H

char	*occurs(const char *big, const char *little);
char	*occurs_exclusively(const char *expected, const char *actual);

char	**split_outside_quotes(const char *to_split, char c);
void	print_arr_sep(char **arr, char sep_open, char sep_close);
char	**append_str_arr(const char **arr, const char *s);
void	rm_str_arr(char **arr, const char *s);
char	**arr_trim(char **arr, char const *set);

# include <stddef.h>
int		arr_ncmp(const char **arr1, const char **arr2, size_t n);

#include <stdbool.h>

bool	*bool_arr_zeroing(size_t len);
void	range_ignore(const char *s, bool *ignore, unsigned char c);
#endif