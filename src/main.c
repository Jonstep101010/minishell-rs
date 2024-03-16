#include "lexer.h"
#include "struct.h"
#include <readline/readline.h>
#include <readline/history.h>
#include <unistd.h>

#include "msh_signals.h"

#include "minishell.h"
#include "libft.h"
#include <stdlib.h>
#include "get_next_line.h"
#include "utils.h"
#include "commands.h"
t_lexer lexer(t_shell *shell, const char *trimmed_line);

void	minishell_loop(t_shell *shell)
{
	char	*readline_line;
	char	*trimmed_line;

	check_signals(&shell->p_termios);
	while (1)
	{
		readline_line = readline("minishell> ");
		trimmed_line = ft_strtrim(readline_line, WHITESPACE);
		if (!readline_line || !trimmed_line)
			builtin_exit(shell, NULL);
		add_history(trimmed_line);
		free(readline_line);
		if (*trimmed_line == '\0' || lexer(shell, trimmed_line) != LEXER_SUCCESS)
		{
			free(trimmed_line);
			continue ;
		}
		free(trimmed_line);
		if (shell->env && *shell->env && shell->token)
		{
			execute_commands(shell, shell->token);
			destroy_all_tokens(shell);
		}
	}
}

int main(int ac, char **av, char **envp)
{
	t_shell		*shell;

	(void)ac;
	(void)av;
	// if (ac > 1 || av[1])
	// 	return (printf("do not pass arguments\n"), 1);
	shell = init_shell(envp);
	if (!shell)
		return (1);
	minishell_loop(shell);
	return (0);
}
