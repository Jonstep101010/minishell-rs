#include "struct.h"
#include "libft.h"

int		parser(t_shell *shell)
{
	if (!shell->line)
		return (EXIT_FAILURE);
	shell->command = ft_split(shell->line, ' ');
	if (shell->command == NULL)
		return (EXIT_FAILURE);
	return (EXIT_SUCCESS);
}
