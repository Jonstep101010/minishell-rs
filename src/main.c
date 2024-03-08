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
t_lexer lexer(t_shell *shell);

void	minishell_loop(t_shell *shell)
{
	check_signals(&shell->p_termios);
	while (1)
	{
		if (isatty(fileno(stdin)))
		{
			shell->line = readline("minishell> ");
			if (!shell->line)
				builtin_exit(shell, NULL);
			if (ft_strlen(shell->line) == 0)
			{
				free(shell->line);
				continue ;
			}
			shell->trimmed_line = ft_strtrim(shell->line, WHITESPACE);
			if (!shell->trimmed_line)
				builtin_exit(shell, NULL);
			add_history(shell->trimmed_line);
			if (*shell->trimmed_line == '\0' || lexer(shell) != LEXER_SUCCESS)
			{
				free(shell->line);
				free(shell->trimmed_line);
				continue ;
			}
			deploy_tokens(shell);
		}
		else
		{
			char *line;
			line = get_next_line(fileno(stdin));
			shell->line = ft_strtrim(line, "\n");
			free(line);
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

void	deploy_tokens(t_shell *shell)
{
	if (shell->owned_envp && *shell->owned_envp)
	{
		free(shell->line);
		free(shell->trimmed_line);
		if (shell->line && shell->trimmed_line)
		{
			if (shell->token && shell->owned_envp)
			{
				execute_commands(shell, shell->token);
				destroy_all_tokens(shell);
			}
		}
	}
}
