#ifndef PARSER_H
# define PARSER_H

# include <stddef.h>
typedef struct s_splitter
{
	int			quote;
	size_t		start;
	size_t		len;
	char		**arr;
	char		*to_split;
	const char	*set;
}	t_splitter;

char	**split_outside_quotes(const char *to_split, const char *set);

# include <stdbool.h>
void	*do_quote_bs(const char *s, int *quote);
bool	interpret_quotes(char **cmd_arr);
#endif