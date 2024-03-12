#ifndef STRUCT_H
# define STRUCT_H

# include <stddef.h>
# include "tokens.h"
# include <stdbool.h>
# include <termios.h>

typedef struct s_shell
{
	char			**split_tokens;
	char			**split_pipes;
	int				exit_status;
	char			**env;
	char			*line;
	char			*trimmed_line;
	char			*expanded_line;
	char			*tmp;
	// char			**tmp_arr;
	// char			**command;// not sure if we need this
	t_token			*token;
	struct termios	p_termios;
}	t_shell;

#endif