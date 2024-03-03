#ifndef MSH_SIGNALS_H
# define MSH_SIGNALS_H
# include <signal.h>
# include <termios.h>
# include <stdbool.h> //@follow-up

// bool	g_ctrl_c = false;

void	check_signals(struct termios *p_termios);
void	ctrl_c_init(void);
void	ctrl_c_handler(int sig, siginfo_t *info, void *unused);
void	ctrl_bkslash_init(void);
#endif