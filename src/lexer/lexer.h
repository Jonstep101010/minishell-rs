#ifndef LEXER_H
# define LEXER_H
# include "struct.h"
t_lexer	check_quotes_ignore(const char *s, struct s_lexer *input);
bool	redir_valid(const char *s, const int redircount, char c);
bool	two_pipes_valid(const char *s, const int index);
bool	pipes_valid(const char *s, const int pipes);
t_lexer	check_pipes_redirection(const char *s, struct s_lexer *input);
t_lexer	check_brackets_quotes(struct s_lexer *input);
t_lexer	check_against_ignore(const char *s, struct s_lexer *input);
void	count_number(char *s, struct s_lexer *input);

# include <stddef.h>
void	bool_arr_zeroing(const char *s, bool **ignore, size_t len);
void	range_ignore(const char *s, bool **ignore, unsigned char c);

#endif // LEXER_H