#include "minishell.h"
#include "libft.h"
#include "struct.h"
#include <readline/readline.h>
#include <readline/history.h>
#include <unistd.h>

#include "../src/signals/msh_signals.h"
#include "../src/builtins/builtins.h"
#include "../src/parser/parser.h"

void	core_functions(t_shell *shell);

#include <sys/param.h>
void	minishell_loop(t_shell *shell)
{
	struct termios p_termios;
	// char	buf[MAXPATHLEN + 1];

	while (1)
	{
		check_signals(&p_termios);
		// getcwd(buf, MAXPATHLEN);
		// ft_printf("%s\n", buf);
		shell->line = readline("minishell> ");
		if (shell->line)
		{
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
			msh_exit(shell, 1);
		if (builtin((const char **)shell->command) == -1)
			ft_printf("command is not a builtin/command not found\n");
		// @todo executor
		arr_free(shell->command);
		add_history(shell->line);
	}
	else
		ft_printf("invalid syntax\n");
}

int main(int ac, char **av, const char **envp)
{
	t_shell		*shell;

	(void)ac;
	(void)av;
	shell = (t_shell *) ft_calloc(1, sizeof(t_shell));
	shell->envp = envp;
	shell->owned_envp = arr_dup(envp);
	if (lexer("echo Hello, World!") != LEXER_SUCCESS)
		ft_printf("lexer does not work\n");
	minishell_loop(shell);
	return (0);
}
