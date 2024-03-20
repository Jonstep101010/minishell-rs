#include "arr_utils.h"
#include "ft_printf.h"
#include "struct.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void	eprint(const char *fmt, ...)
{
	va_list	args;

	va_start(args, fmt);
	write(STDERR_FILENO, "minishell: ", 11);
	ft_vdprintf(STDERR_FILENO, fmt, args);
	write(STDERR_FILENO, "\n", 1);
	va_end(args);
}

void	eprint_single(const char *fmt, ...)
{
	va_list	args;

	va_start(args, fmt);
	ft_vdprintf(STDERR_FILENO, fmt, args);
	va_end(args);
}

void	exit_free(t_shell *shell, int exit_code)
{
	if (exit_code == 0)
		return ;
	if (shell->env)
		arr_free(shell->env);
	destroy_all_tokens(shell);
	free(shell);
	exit(exit_code);
}
