#ifndef EXPANDER_H
# define EXPANDER_H
# include <stddef.h>

typedef struct s_expand
{
	char	*expanded;
	char	*after_expansion;
	char	*tmp;
	char	*to_expand;
	char	*before_expansion;
	size_t	tmp_len;
	size_t	i;
	size_t	end;
	int		singlequote;
}	t_expand;

char	*expander(char const *input_expander, char *const *env);

#endif