#ifndef STRUCT_H
# define STRUCT_H

#include <stdbool.h>
typedef enum e_lexer
{
	LEXER_SUCCESS,
	LEXER_NULL,
	LEXER_UNBALANCED_QUOTES,
	LEXER_UNBALANCED_BRACKETS,
	LEXER_REDIRECTION,
	LEXER_PIPES,
	// LEXER_UNBALANCED_PARENTHESIS,
	// LEXER_UNBALANCED_SINGLE_QUOTES,
	// LEXER_UNBALANCED_DOUBLE_QUOTES,
	// LEXER_UNBALANCED_CURLY_BRACKETS,
	// LEXER_UNBALANCED_SQUARE_BRACKETS,
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
};

#endif