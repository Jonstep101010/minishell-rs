#include <termios.h>
#include "msh_signals.h"
#include <signal.h>
#include <stddef.h>

static void	ctrl_c_child(void);
static void	ctrl_backslash_child(void);

void	check_signals_child(struct termios *p_termios_child)
{
	struct termios	attr;

	attr = (struct termios){0};
	tcgetattr(0, p_termios_child);
	tcgetattr(0, &attr);
	attr.c_lflag &= ~ECHOCTL;
	tcsetattr(1, TCSAFLUSH, p_termios_child);
	ctrl_backslash_child();
	ctrl_c_child();
}

static void	ctrl_c_child(void)
{
	struct sigaction	ctrl_c;

	ctrl_c.sa_handler = SIG_DFL;
	ctrl_c.sa_flags = SA_RESTART;
	sigemptyset(&ctrl_c.sa_mask);
	sigaction(SIGINT, &ctrl_c, NULL);
}

static void	ctrl_backslash_child(void)
{
	struct sigaction	ctrl_slash;

	ctrl_slash.sa_handler = SIG_DFL;
	ctrl_slash.sa_flags = SA_RESTART;
	sigemptyset(&ctrl_slash.sa_mask);
	sigaction(SIGQUIT, &ctrl_slash, NULL);
}
