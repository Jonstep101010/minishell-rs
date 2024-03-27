#include "ft_printf.h"
#include <stdio.h>
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

