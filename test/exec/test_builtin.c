#include "unity.h"
#include <string.h>
#include <unistd.h>

typedef struct s_token	t_token;

enum e_token_type
{
	PIPE,
	SEMICOLON,
	REDIR_IN,
	REDIR_OUT,
	REDIR_APPEND,
	REDIR_FD,
	COMMAND,
	ARG,
	STRING,
};

typedef struct s_args
{
	char	*elem;
	enum e_token_type	type;
}	t_args;

typedef struct s_cmd
{
	char	*elem;
	enum e_token_type	type;
}	t_cmd;

struct s_token
{
	t_cmd	*cmd;
	t_args	*cmd_args;
	char	**command;
	char	**args;
	int		type;
	int		(*func)(t_token *);
};

int	echo(char **cmd, char **args, char **envp)
{
	(void)cmd;
	(void)envp;
	printf("echo\n");
	if (args)
		printf("%s\n", *args);
	return (0);
}

typedef int (*t_builtin_func)(char **cmd, char **args, char **envp);

typedef struct s_builtin
{
	char *name;
	t_builtin_func func;
}	t_builtin;

static const t_builtin builtins[] = {{"echo", (t_builtin_func)echo}, {NULL, NULL}};

typedef struct s_shell
{
	int		exit_status;
	char	**env;
	char	*line;
	char	*expanded_line;
	char	*tmp;
	char	**tmp_arr;
	char	**command;
	char	**tokens_tmp;
	t_token	*token;
}	t_shell;

// int execute_command(char *name, char **args) {
//   // Check if the command is a built-in command
//   for (int i = 0; builtins[i].name; i++) {
// 	if (strcmp(builtins[i].name, name) == 0)
// 	  return builtins[i].func(args);
//   }

//   // If the command is not a built-in command, execute it as an external program
//   execve(name, args, NULL);

//   // If execve returns, there was an error
//   perror("execve");
//   return -1;
// }

// int		builtin(char **cmd, char **args)
// {
// 	// for (int i = 0; builtins[i].name; i++)
// 	// {
// 	// 	if (strcmp(builtins[i].name, shell->command[0]) == 0)
// 	// 		return builtins[i].func(shell->command);
// 	// }
// 	// return (execute_command(shell->command[0], &shell->command[1]));
// 	return (0);
// }

void test_builtin()
{
	t_shell shell;
	shell.command = (char *[]){"echo", "hello", NULL};
	TEST_ASSERT_EQUAL_INT(0, builtins[0].func(shell.command, &shell.command[1], NULL));
}
