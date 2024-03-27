#ifndef ENV_H
# define ENV_H
# include <stdbool.h>
# include "struct.h"

int		get_index_env(char *const *env, char const *substr);
char	*get_env(char *const *env, char const *key);

bool	check_valid_key(const char *s);

void	export_env(t_shell *shell, char *key_val);

char	*expander(char const *input_expander, char *const *env);

#endif