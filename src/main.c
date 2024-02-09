#include "ft_printf.h"
#include "libft.h"
#include "struct.h"
#include <readline/readline.h>
#include <readline/history.h>
#include <unistd.h>

#include "msh_signals.h"
void	get_tokens(t_shell *shell);
int		builtin(t_shell *shell, t_token *token);
void	execute_commands(t_shell *shell, t_token *token);
void	destroy_all_tokens(t_shell *shell);

void	minishell_loop(t_shell *shell)
{
	while (1)
	{
		get_tokens(shell);
		if (shell->token)
		{
			builtin(shell, shell->token);
			destroy_all_tokens(shell);
		}
		else if (shell->exit_status == 2)
			continue;
		// does quitting using ctrl_d cause leaks?
		else
			return ;
	}
}

int main(int ac, char **av, char **envp)
{
	t_shell		*shell;

	(void)ac;
	(void)av;
	shell = (t_shell *) ft_calloc(1, sizeof(t_shell));
	shell->owned_envp = arr_dup((const char **)envp);
	minishell_loop(shell);
	return (0);
}

t_token	*lexer(t_shell *shell);

void	tokenize(t_shell *shell)
{
	shell->token = lexer(shell);
	if (!shell->token)
		return ;// some exit code
	// handle exit code for failed parsing/lexical errors
}
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
		tokenize(shell);
		free(shell->line);
	}
}
