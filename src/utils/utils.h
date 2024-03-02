#ifndef UTILS_H
# define UTILS_H
# include "arr_utils.h"
# include "libutils.h"

char	*occurs(const char *big, const char *little);
char	*occurs_exclusively(const char *expected, const char *actual);

char	**split_outside_quotes(const char *to_split, const char *set);
void	print_arr_sep(char **arr, char sep_open, char sep_close);
void	rm_str_arr(char **arr, const char *s);
char	**arr_trim(char **arr, char const *set);

void	update_variable(char **envp, const char *key, const char *value);
char	*free_strjoin(int count, ...);
void	free_n(int n, ...);

# include <stddef.h>
int		arr_ncmp(const char **arr1, const char **arr2, size_t n);

#include <stdbool.h>

bool	*bool_arr_zeroing(size_t len);
void	range_ignore(const char *s, bool *ignore, unsigned char c);

// error handling

/**
 * @brief "minishell: " + fmt on stderr
 */
void	eprint(const char *fmt, ...);

/**
 * @brief fmt on stderr
 */
void	eprint_single(const char *fmt, ...);
#endif