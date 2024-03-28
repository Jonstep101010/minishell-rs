#ifndef LEXER_H
# define LEXER_H
# include <stddef.h>
# include <stdbool.h>

# define LEXER_SUCCESS 0
// enum e_lexer
// {
	// LEXER_SUCCESS,
// 	LEXER_NULL,
// 	LEXER_UNBALANCED_QUOTES,
// 	LEXER_UNBALANCED_BRACKETS,
// 	LEXER_REDIRECTION,
// 	LEXER_PIPES,
// 	LEXER_SINGLE_QUOTE,
// 	LEXER_DOUBLE_QUOTE,
// 	ERR_TRAILING,
// 	ERR_LEADING,
// 	ERR_EMPTY_PIPES,
// 	LEXER_BEGIN,
// };

typedef struct s_lexer
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
	bool	*ignore;
	size_t	len;
	char	*error;
	int		 lexer;
	bool	result;
}	t_lexer;

int				ignore_quotes(const char *s, struct s_lexer *input);
int				check_pipes_redirection(const char *s, t_lexer *input);
void			count_number(const char *s, struct s_lexer *input);

bool			*bool_arr_zeroing(size_t len);
void			range_ignore(const char *s, bool *ignore, unsigned char c);

#endif // LEXER_H