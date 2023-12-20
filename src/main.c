#include "minishell.h"
#include <signal.h>

// @follow-up implement calls to readline library
void	minishell_loop(void)
{
	char *line;

	while (1)
	{
		line = readline("minishell> ");
		if (line)
		{
			if (ft_strncmp(line, "exit", 4) == 0)
			{
				free(line);
				printf("exiting now...\n");
				exit(0);
			}
			if (lexer(line) == LEXER_SUCCESS)
				printf(":: %s\n", line);
			else
				printf("invalid syntax\n");
			free(line);
		}
	}
}

int main()
{
	// minishell_loop();
	printf("As beautiful as a shell!\n");
	if (lexer("echo Hello, World!") == LEXER_SUCCESS)
		printf("lexer works\n");
	else
		printf("lexer does not work\n");
	minishell_loop();
	return (0);
}
