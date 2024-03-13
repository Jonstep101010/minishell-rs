#ifndef ENV_H
# define ENV_H

int		get_index_env(char *const *env, char const *substr);
char	*get_env(char *const *env, char const *key);

#include <stdbool.h>
bool	check_valid_key(const char *s);

# include "struct.h"
void	export_env(t_shell *shell, char *key_val);

void	update_exit_status(t_shell *shell, int status);

char	*expander(char const *input_expander, char *const *env);

#endif