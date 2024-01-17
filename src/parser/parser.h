#ifndef PARSER_H
# define PARSER_H

# include "struct.h"
int		parser(t_shell *shell);
char	*occurs(const char *big, const char *little);
char	*occurs_exclusively(const char *expected, const char *actual);
#endif