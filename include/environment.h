#ifndef ENV_H
# define ENV_H

// utils for finding kv
#include <stddef.h>
size_t	get_key_len(const char *s);
int		find_key_env(char *const *env, char const *s, size_t (*f)(char const *s));

#include <stdbool.h>
bool	check_valid_key(const char *s);
char	**export_var(char **env, const char *key_val);
# include "struct.h"
void	export_to_shell(t_shell *shell, char *key_val);
char	*get_env_var(char *const *env, char const *key);

#endif