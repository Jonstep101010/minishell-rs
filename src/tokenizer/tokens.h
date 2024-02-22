#ifndef TOKENS_H
# define TOKENS_H
typedef struct s_token	t_token;

# ifndef WHITESPACE
#  define WHITESPACE " \t\n\r\v\f"
# endif

enum e_arg
{
	STRING,
	PIPE,
	EXPANSION,
};

enum e_quote
{
	SINGLE,
	DOUBLE,
	NONE,
};

enum e_builtin
{
	BUILTIN_ECHO,
	CD,
	PWD,
	EXPORT,
	UNSET,
	ENV,
	EXIT,
	NOT_BUILTIN,
};

typedef struct s_arg
{
	char	*elem;
	enum	e_arg	type;
	enum	e_quote	quote;
}	t_arg;

typedef struct s_shell	t_shell;
struct s_token
{
	t_arg	*cmd_args;// keep attributes in execution (i.e. redirs), cmd_args[0] is the first token/command (not pipe)
	char	*split_pipes;
	// char	**args;
	char	**tmp_arr;
	char	**command;// for execution (each token has the command)
	enum	e_builtin	builtin_info;
	// size_t	status;// for usage with the pipes?
	// char	*bin;// for finding path/to/bin?
	int		(*cmd_func)(t_shell *, t_token *);// not sure if this is necessary
};
# endif