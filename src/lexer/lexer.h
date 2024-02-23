#ifndef LEXER_H
# define LEXER_H

# include <stddef.h>
# include <stdbool.h>
typedef enum e_lexer
{
	LEXER_SUCCESS,
	LEXER_NULL,
	LEXER_UNBALANCED_QUOTES,
	LEXER_UNBALANCED_BRACKETS,
	LEXER_REDIRECTION,
	LEXER_PIPES,
	LEXER_SINGLE_QUOTE,
	LEXER_DOUBLE_QUOTE,
}	t_lexer;

struct s_lexer
{
	int		singlequotes;
	int		doublequotes;
	int		open_curly_brackets;
	int		close_curly_brackets;
	int		open_square_brackets;
	int		close_square_brackets;
	int		open_parentheses;
	int		close_parentheses;
	int		redir_greater;
	int		redir_smaller;
	int		pipes;
	t_lexer	lexer;
	bool	*ignore;
	size_t	len;
};

t_lexer	ignore_quotes(const char *s, struct s_lexer *input);
bool	redir_valid(const char *s, const int redircount, char c);
bool	two_pipes_valid(const char *s, const int index);
bool	pipes_valid(const char *s, const int pipes);
t_lexer	check_pipes_redirection(const char *s, struct s_lexer *input);
t_lexer	check_against_ignore(const char *s, struct s_lexer *input);
void	count_number(const char *s, struct s_lexer *input);

# include <stddef.h>
bool	*bool_arr_zeroing(size_t len);
void	range_ignore(const char *s, bool *ignore, unsigned char c);

#endif // LEXER_H