#ifndef PARSER_H
# define PARSER_H

# include <stdbool.h>
void	*do_quote_bs(const char *s, int *quote);
bool	interpret_quotes(char **cmd_arr);
#endif