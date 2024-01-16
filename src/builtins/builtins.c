#include "builtins.h"
#include "struct.h"
#include "libft.h"
#include <stdbool.h>
#include <stdlib.h>

char	*occurs_exclusively(const char *, const char *);
// @follow-up parser needs to run before builtins in future,
// pass in only command char **
// need to add
int		builtin(const char **cmd_arr)
{
	if (!cmd_arr || !*(cmd_arr))
		return (-1);
	print_arr(cmd_arr);
	if (occurs_exclusively("echo", *cmd_arr))
		return (echo(cmd_arr));
	return (-1);
}
