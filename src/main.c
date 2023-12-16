#include "../include/minishell.h"

// @follow-up implement calls to readline library
// void	minishell_loop(void)
// {
// 	while (1)
// 	{
// 		read_line(char *buf, int fd, int *counter, char **line)
// 	}
// }

int main()
{
	// minishell_loop();
	printf("As beautiful as a shell!\n");
	if (lexer("echo \"Hello, World!\"") == LEXER_SUCCESS)
		printf("lexer works\n");
	else
		printf("lexer does not work\n");
	return (0);
}
