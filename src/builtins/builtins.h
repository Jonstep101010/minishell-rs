#ifndef BUILTINS_H
# define BUILTINS_H

# include "struct.h"

void	add_pipes_as_tokens(t_shell *shell);
void	convert_split_token_string_array_to_tokens(t_shell *shell);
void	convert_tokens_to_string_array(t_token *token);
void	destroy_all_tokens(t_shell *shell);

#endif