#ifndef PARSER_H
# define PARSER_H

# include <stddef.h>
typedef struct s_splitter
{
	int		quote;
	size_t	i;
	size_t	start;
	size_t	token_end;
	size_t	len;
	char	**ret;
	char	*tmp;
	char	*tmp2;
	char	**arr;
	char	**not_last_token;
	size_t	trim;
}	t_splitter;

# include <stdbool.h>
void	*do_quote_bs(const char *s, int *quote);
bool	interpret_quotes(char **cmd_arr);
#endif