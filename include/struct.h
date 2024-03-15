#ifndef STRUCT_H
# define STRUCT_H

# include "tokens.h"
# include <stdbool.h>
# include <stdint.h>
# include <termios.h>

typedef struct s_shell
{
	char			**split_pipes;
	uint8_t			exit_status;
	char			**env;
	char			*line;
	char			*trimmed_line;
	t_token			*token;
	struct termios	p_termios;
}	t_shell;

#endif