#include "unity.h"
#include "libft.h"
#include "struct.h"

t_shell	*setup_shell(char *line) {
	t_shell *shell = (t_shell *) calloc(sizeof(t_shell), 1);
	shell->line = strdup(line);
	if (!shell)
		TEST_FAIL_MESSAGE("Failed to allocate memory for shell");
	if (!shell->line)
		TEST_FAIL_MESSAGE("Failed to allocate memory for line");
	return (shell);
}

void	clean_shell(t_shell *shell) {
	if (shell->line)
		free(shell->line);
	if (shell->command)
		arr_free(shell->command);
	free_null(shell);
}

void	debug_print_arr(t_shell *this)
{
	for (int i = 0; this->command[i]; i++)
	{
		fprintf(stderr, "string in arr:%s\n", this->command[i]);
	}
}
