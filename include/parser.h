#ifndef PARSER_H
# define PARSER_H

# include <stddef.h>
typedef struct s_splitter
{
	int		quote;
	size_t	i;
	size_t	start;
	size_t	token_end;
	char	*tmp;
	char	*last;
	char	**arr;
}	t_splitter;

char	**split_outside_quotes(const char *to_split, const char *set);

# include <stdbool.h>
void	*do_quote_bs(const char *s, int *quote);
bool	interpret_quotes(char **cmd_arr);
#endif