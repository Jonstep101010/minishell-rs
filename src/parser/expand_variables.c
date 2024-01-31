#include "env.h"
#include "libft.h"
#include <stdbool.h>

static bool	is_valid_key(int c)
{
	if (ft_isalnum(c) || c == '_')
		return (true);
	return (false);
}

char	*expand_variables(const char *input, const char **envp)
{
	size_t	i;
	char	*ret;
	char	*key;
	size_t	start;
	char	*val;
	char	*tmp;
	int		singlequote;
	char	*remainder_line;

	i = 0;
	singlequote = 0;
	char	*line = ft_strdup(input);
	if (!line)
		return (NULL);
	while (line[i])
	{
		if (line[i] == '\'' && singlequote == 0)
			singlequote = line[i];
		else if (line[i] == '\'' && singlequote == line[i])
			singlequote = 0;
		else if (line[i] && line[i + 1] && line[i] == '$'
			&& singlequote == 0 && is_valid_key(line[i + 1]))
		{
			i++;
			start = i;
			// iterate over key
			while (line[i] && is_valid_key(line[i]))
				i++;
			// if no key, continue looping
			if (start == i)
				continue;
			// get key
			key = ft_substr(line, start, i - start);
			if (!key)
				return (NULL);
			val = get_var_val(envp, key);
			free(key);
			if (!val)
				return (NULL);
			tmp = ft_substr(line, 0, start - 1);
			if (!tmp)
				return (free(val), NULL);
			ret = ft_strjoin(tmp, val);
			free(tmp);
			free(val);
			if (!ret)
				return (NULL);
			// if not done looping the line -> replace ret after line index with line
			if (i <= ft_strlen(line))
			{
				remainder_line = ft_strdup(&line[i]);
				free(line);
				if (!remainder_line)
					return (ret);
				printf("remainder: %s\n", remainder_line);
				tmp = ft_strjoin(ret, remainder_line);
				free(remainder_line);
				free(ret);
				ret = expand_variables(tmp, envp);
				free(tmp);
			}
			printf("expanded now: %s\n", ret);
			return (ret);
		}
		if (line[i] == '$' && singlequote == 0)
			line[i] = ' ';
		i++;
	}
	return (line);
}
