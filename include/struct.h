#ifndef STRUCT_H
# define STRUCT_H

# include "tokens.h"
# include <stdbool.h>
# include <stdint.h>
# include <termios.h>

typedef struct s_shell
{
	uint8_t			exit_status;
	char			**env;
	t_token			*token;
	struct termios	p_termios;
}	t_shell;

void	update_exit_status(t_shell *shell, int status);

#endif