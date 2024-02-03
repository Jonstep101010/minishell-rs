#include "ft_printf.h"
#include "minishell.h"
#include "libft.h"
#include "struct.h"
#include <readline/readline.h>
#include <readline/history.h>
#include <unistd.h>

void	msh_exit(t_shell *shell, int exitcode)
{
	if (shell->line)
		free_null(shell->line);
	ft_printf("exiting now...\n");
	if (shell->owned_envp)
		arr_free(shell->owned_envp);
	free_null(shell);
	exit(exitcode);
}

#include "../src/signals/msh_signals.h"
#include "../src/builtins/builtins.h"
#include "../src/parser/parser.h"

void	core_functions(t_shell *shell);

void	minishell_loop(t_shell *shell)
{
	struct termios p_termios;

	while (1)
	{
		check_signals(&p_termios);
		shell->line = readline("minishell> ");
		if (shell->line)
		{
			add_history(shell->line);
			core_functions(shell);
		}
		// does quitting using ctrl_d cause leaks?
		else
			return ;
	}
}

void	core_functions(t_shell *shell)
{

	if (ft_strncmp(shell->line, "exit", 4) == 0 && ft_strlen(shell->line) == 4)
		msh_exit(shell, 0);
	else if (lexer(shell->line) == LEXER_SUCCESS)
	{
		// @todo modify parser to format correctly and return error codes
		if (parser(shell) == -1)
			ft_printf("I am not segfaulting (yet)\n");
		if (builtin(shell) == -1)
			ft_printf("command is not a builtin/command not found\n");
		// @todo executor
		if (shell->command)
			arr_free(shell->command);
		free_null(shell->line);
	}
	else
		ft_printf("invalid syntax\n");
}

int main(int ac, char **av, char **envp)
{
	t_shell		*shell;

	(void)ac;
	(void)av;
	shell = (t_shell *) ft_calloc(1, sizeof(t_shell));
	shell->owned_envp = arr_dup((const char **)envp);
	if (!shell->owned_envp)
		exit(1);
	if (lexer("echo Hello, World!") != LEXER_SUCCESS)
		ft_printf("lexer does not work\n");
	minishell_loop(shell);
	return (0);
}
