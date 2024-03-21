#ifndef TOKENS_H
# define TOKENS_H
# include <stddef.h>
# include <stdbool.h>
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
	REDIR,
	REDIR_REMOVED,
};

typedef struct s_arg
{
	char	*elem;
	enum	e_arg	type;
	enum	e_redir	redir;
}	t_arg;

struct s_token
{
	t_arg	*cmd_args;
	bool	has_redir;
	char	*split_pipes;
	char	**tmp_arr;
	char	**command;
	char	*bin;
	int		(*cmd_func)(t_shell *, t_token *);
};

void	set_cmd_func(const char *cmd, t_token *token);

t_token	*get_tokens(char const *trimmed_line);
void	*tokenize(t_shell *shell, char const *trimmed_line);
void	convert_tokens_to_string_array(t_token *token);
void	destroy_all_tokens(t_shell *shell);
t_arg	*init_cmdargs(size_t size);
t_token	*init_token(size_t size);
# endif
