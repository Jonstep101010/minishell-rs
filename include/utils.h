#ifndef UTILS_H
# define UTILS_H
# include "arr_utils.h"
# include "libutils.h"
#include "tokens.h"

char	*equal(const char *expected, const char *actual);

char	**split_outside_quotes(const char *to_split, const char *set);
void	print_arr_sep(char **arr, char sep_open, char sep_close);
void	rm_str_arr(char **arr, const char *s);
char	**arr_trim(char **arr, char const *set);

// general utils
char	*free_strjoin(int count, ...);
void	free_n(int n, ...);
void	free_n_null(int n, ...);
char	*str_join(const int count, ...);
char	*free_second_join(char const *s1, char *s2);

# include <stddef.h>
int		arr_ncmp(char *const *arr1, char *const *arr2, size_t n);

#include <stdbool.h>

bool	*bool_arr_zeroing(size_t len);
void	range_ignore(const char *s, bool *ignore, unsigned char c);

// error handling

/**
 * @brief "minishell: " + fmt on stderr
 */
void	eprint(const char *fmt, ...);
void	exit_free(t_shell *shell, int exit_code);
void	exit_error(t_shell *shell, char *error_elem);

/**
 * @brief fmt on stderr
 */
void	eprint_single(const char *fmt, ...);
# include "struct.h"
t_shell	*init_shell(char *const *envp);
void	update_exit_status(t_shell *shell, int status);


#endif