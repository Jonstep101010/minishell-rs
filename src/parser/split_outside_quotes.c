#include "libft.h"
#include "utils.h"
#include "arr_utils.h"
#include <unistd.h>
#include "parser.h"

static char	**split_loop(t_splitter *s)
{
	size_t	i;

	i = 0;
	while (++i < s->len)
	{
		if (s->quote && s->to_split[i] == s->quote)
			s->quote = 0;
		else if (!s->quote && ft_strchr("'\"", s->to_split[i]))
			s->quote = s->to_split[i];
		if (!s->quote && ft_strchr(s->set, s->to_split[i]))
		{
			s->arr = append_str_arr_free(s->arr,
					ft_substr(s->to_split, s->start, i - s->start));
			if (!s->arr)
				return (NULL);
			while (s->to_split[i + 1]
				&& ft_strchr(s->set, s->to_split[i + 1]))
				i++;
			s->start = i + 1;
		}
	}
	return (append_str_arr_free(s->arr,
			ft_strtrim(&s->to_split[s->start], s->set)));
}

char	**split_outside_quotes(const char *to_split, const char *set)
{
	char		**ret;
	t_splitter	s;

	if (!to_split)
		return (NULL);
	s = (t_splitter){0, 0, 0, NULL, ft_strtrim(to_split, set), set};
	if (!s.to_split)
		return (NULL);
	s.len = ft_strlen(s.to_split);
	ret = split_loop(&s);
	free(s.to_split);
	return (ret);
}
