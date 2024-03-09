#ifndef ENV_H
# define ENV_H

int		get_index_env(char *const *env, char const *substr);

#include <stdbool.h>
bool	check_valid_key(const char *s);
char	**export_var(char **env, const char *key_val);
# include "struct.h"
void	export_to_shell(t_shell *shell, char *key_val);
char	*get_env(char *const *env, char const *key);

#endif