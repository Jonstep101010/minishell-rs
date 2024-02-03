#ifndef ENV_H
# define ENV_H

// utils for finding kv
#include <stddef.h>
size_t	get_key_len(const char *s);
int		find_key_env(const char **arr, const char *s, size_t (*f)(const char *s));

char	**export_var(char **arr, const char *s);

char	**unset(char **arr, const char *s);

char	*get_var_val(const char **arr, const char *key);

#endif