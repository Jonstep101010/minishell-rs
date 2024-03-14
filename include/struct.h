#ifndef STRUCT_H
# define STRUCT_H

# include "tokens.h"
# include <stdbool.h>
# include <stdint.h>
# include <termios.h>

typedef struct s_shell
{
	char			**split_tokens;
	char			**split_pipes;
	uint8_t			exit_status;
	char			**env;
	char			*line;
	char			*trimmed_line;
	char			*expanded_line;
	char			*tmp;
	t_token			*token;
	struct termios	p_termios;
}	t_shell;

#endif