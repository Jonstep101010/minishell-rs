#include "libft.h"
#include "libutils.h"
#include "tokens.h"
#include "utils.h"

static char	*collect_as_ascii(char *readline_line)
{
	int		i;
	char	*collected_line;

	i = 0;
	collected_line = NULL;
	while (readline_line && readline_line[i])
	{
		if (ft_isascii(readline_line[i]))
		{
			collected_line = append_char_str(collected_line, readline_line[i]);
			if (!collected_line)
				return (NULL);
		}
		i++;
	}
	if (readline_line)
		free(readline_line);
	return (collected_line);
}

char	*get_input(char *rl_prompt)
{
	static	char	*line;
	char			*trim;

	if (!rl_prompt)
		return (free_null(&line), NULL);
	else
	{
		line = collect_as_ascii(rl_prompt);
		if (!line)
			return (free(rl_prompt), NULL);
		trim = ft_strtrim(line, WHITESPACE);
		if (!trim)
			return (free_null(&line), NULL);
		free_null(&line);
		line = trim;
	}
	return (line);
}
