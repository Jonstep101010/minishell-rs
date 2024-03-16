#ifndef TOKENS_H
# define TOKENS_H
# include <stddef.h>
typedef struct s_token	t_token;
typedef struct s_shell	t_shell;

# ifndef WHITESPACE
#  define WHITESPACE " \t\n\r\v\f"
# endif

enum e_redir
{
	NO_REDIR,
	INPUT_REDIR,
	OUTPUT_REDIR,
	APPEND,
	HEREDOC,
};

enum e_arg
{
	STRING,
	REDIR,//this could prove useful
	// REDIR_WORD,
	REDIR_REMOVED,// remove current t_arg and set to next t_arg
	// (should be REDIR_WORD) - later removed
};

enum e_quote
{
	SINGLE,
	DOUBLE,
	NONE,
};

typedef struct s_arg
{
	char	*elem;
	enum	e_arg	type;
	enum	e_quote	quote;//mostly unused
	enum	e_redir	redir;
}	t_arg;

struct s_token
{
	t_arg	*cmd_args;// keep attributes in execution (i.e. redirs), cmd_args[0] is the first token/command (not pipe)
	char	*split_pipes;
	char	**tmp_arr;
	char	**command;// for execution (each token has the command)
	// char	*bin;// for finding path/to/bin?
	int		(*cmd_func)(t_shell *, t_token *);
};

void	set_cmd_func(t_token *token);

t_token	*get_tokens(char const *trimmed_line);
void	*tokenize(t_shell *shell, char const *trimmed_line);
void	convert_tokens_to_string_array(t_token *token);
void	destroy_all_tokens(t_shell *shell);
t_arg	*init_cmdargs(size_t size);
t_token	*init_token(size_t size);
# endif