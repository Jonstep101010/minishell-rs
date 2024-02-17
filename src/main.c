#include "struct.h"
#include <readline/readline.h>
#include <readline/history.h>
#include <unistd.h>

#include "msh_signals.h"

#include "minishell.h"
#include "libft.h"
#include <stdlib.h>

void	minishell_loop(t_shell *shell)
{
	while (1)
	{
		if (isatty(fileno(stdin)))
		{
			get_tokens(shell);
			if (shell->token)
			{
				update_exit_status(shell, builtin(shell, shell->token));
				destroy_all_tokens(shell);
			}
			// else if (shell->exit_status == 2)
			// 	continue;
			// // does quitting using ctrl_d cause leaks?
			else
				return ;
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


int main(int ac, char **av, const char **envp)
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

t_token	*lexer(t_shell *shell);

#include "utils.h"
#include "builtins.h"
void	get_tokens(t_shell *shell)
{
	shell->line = readline("minishell> ");
	if (shell->line && *shell->line)
	{
		check_signals(&(shell->p_termios));
		if (occurs_exclusively("exit", shell->line))
			return (builtin_exit(shell, 0));
		add_history(shell->line);
		shell->token = lexer(shell);
		if (!shell->token)
			return ;// some exit code
		// handle exit code for failed parsing/lexical errors
		free(shell->line);
	}
}
