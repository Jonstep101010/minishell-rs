#ifndef MSH_SIGNALS_H
# define MSH_SIGNALS_H
# include <signal.h>
# include <termios.h>
# include <stdbool.h> //@follow-up

extern int	g_ctrl_c;

void	check_signals(struct termios *p_termios);
void	check_signals_child(struct termios *p_termios_child);
#endif