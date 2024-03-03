#ifndef ENV_H
# define ENV_H

void	update_variable(char **envp, const char *key, const char *value);
// utils for finding kv
#include <stddef.h>
size_t	get_key_len(const char *s);
int		find_key_env(const char **arr, const char *s, size_t (*f)(const char *s));

#include <stdbool.h>
bool	check_valid_key(const char *s);
char	**export_var(char **arr, const char *s);

char	*get_env_var(char **arr, const char *key);

#endif