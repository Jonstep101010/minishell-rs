#include "minishell.h"
#include "libft.h"
#include <readline/readline.h>
#include <stdio.h>
#include <readline/history.h>
#include "../src/signals/msh_signals.h"

// @follow-up implement calls to readline library
void	minishell_loop(void)
{
	char *line;
	struct termios p_termios;

	while (1)
	{
		check_signals(&p_termios);
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
			add_history(line);
			free(line);
		}
		else
			return ;
	}
}

int main()
{
	printf("As beautiful as a shell!\n");
	if (lexer("echo Hello, World!") == LEXER_SUCCESS)
		printf("lexer works\n");
	else
		printf("lexer does not work\n");
	minishell_loop();
	return (0);
}
