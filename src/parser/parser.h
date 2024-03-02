#ifndef PARSER_H
# define PARSER_H

# include <stddef.h>
typedef struct s_splitter
{
	int		quote;
	size_t	i;
	size_t	start;
	size_t	token_end;
	char	**ret;
	char	*tmp;
	char	**arr;
}	t_splitter;

# include <stdbool.h>
void	*do_quote_bs(const char *s, int *quote);
bool	interpret_quotes(char **cmd_arr);
#endif