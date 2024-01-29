#ifndef MSH_SIGNALS_H
# define MSH_SIGNALS_H

// #include <stdbool.h> //@follow-up
// bool	g_ctrl_c	= false;
#include <signal.h>
#include <termios.h>
void	check_signals(struct termios *p_termios);
void	ctrl_c_init(void);
void	ctrl_c_handler(int sig, siginfo_t *info, void *unused);
void	ctrl_bkslash_init(void);
#endif