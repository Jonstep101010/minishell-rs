/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   signals.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 20:02:37 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 20:02:38 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <signal.h>
#include <termios.h>
#include <unistd.h>
#include <stdio.h>
#include <readline/readline.h>
#include "msh_signals.h"

static void	ctrl_bkslash_init(void);

static void	ctrl_c_init(void);

static void	ctrl_c_handler(int sig, siginfo_t *info, void *unused);

void	check_signals(struct termios *p_termios)
{
	tcgetattr(0, p_termios);
	p_termios->c_lflag &= ~ECHOCTL;
	tcsetattr(0, TCSAFLUSH, p_termios);
	ctrl_c_init();
	ctrl_bkslash_init();
}

static void	ctrl_bkslash_init(void)
{
	struct sigaction	sig;

	sig.sa_handler = SIG_IGN;
	sig.sa_flags = SA_RESTART;
	sigemptyset(&(sig.sa_mask));
	sigaction(SIGQUIT, &sig, NULL);
}

static void	ctrl_c_init(void)
{
	struct sigaction	sig;

	sig.sa_sigaction = ctrl_c_handler;
	sig.sa_flags = SA_RESTART;
	sigemptyset(&(sig.sa_mask));
	sigaction(SIGINT, &sig, NULL);
}

static void	ctrl_c_handler(int sig, siginfo_t *info, void *unused)
{
	(void)unused;
	(void)info;
	if (sig == SIGINT)
	{
		g_ctrl_c = 1;
		write(0, "\n", 1);
		rl_on_new_line();
		rl_replace_line("", 0);
		rl_redisplay();
	}
}
