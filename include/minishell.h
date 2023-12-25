#ifndef MINISHELL_H
# define MINISHELL_H
# include "struct.h"
# include <readline/readline.h>
/* helper functions to prototype*/
int while_d(const char *s, int (*)(int), int is_true, int *index);
int while_i(const char *s, int (*)(int), int is_true, int *index);
int	while_not_i(const char *s, int (*)(int), char c, int *index);
int	while_is_i(const char *s, char c, int *index);


void	range_ignore(const char *s, bool **ignore, unsigned char c);
t_lexer	lexer(char *s);
#endif
