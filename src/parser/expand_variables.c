#include "ft_printf.h"
#include "struct.h"
#include "env.h"
#include "libft.h"
#include <ctype.h>
#include <stdbool.h>


static bool	is_valid_key(int c)
{
	if ((c >= 'A' && c <= 'Z') || c == '_')
		return (true);
	return (false);
}

// testing

// echo hello$PAGER -> echo helloVAL
// echo "$PAGER" -> echo VAL
// echo $PAGER -> echo VAL
// echo $'PAGER'S -> echo PAGERS (remove $)
// echo '$PAGER' -> echo $PAGER
// echo $"PAGER"S -> echo PAGERS
// echo $PAGER_S -> echo VAL

// envp should be const (in get_var_val as well)
// will replace valid keys that are not found with empty strings
char	*expand_variables(char *line, char **envp)
{
	size_t	i;
	char	*ret;
	char	*key;
	size_t	start;
	char	*val;
	char	*tmp;
	char	*remainder_line;
	char	*new_ret;
	int		singlequote = 0;

	if (ft_strchr(line, '$') == 0)
		return (line);
	i = 0;
	// ret = ft_calloc(ft_strlen(line) + 1, sizeof(char));
	// if (!ret)
	// 	return (NULL);
	while (line[i])
	{
		if (line[i] == '\'' && singlequote == 0)
			singlequote = line[i];
		else if (line[i] == '\'' && singlequote == line[i])
			singlequote = 0;
		// @follow-up while in doublequotes (& uppercase or underscore) or if space afterwards
		else if (line[i] && line[i + 1] && line[i] == '$' && singlequote == 0
				&& is_valid_key(line[i + 1]))
		{
			// ONLY UPPERCASE/UNDERSCORE!
			i++;
			start = i;
			while (line[i] && is_valid_key(line[i]))
				i++;
			if (start == i)
				continue;;
			key = ft_substr(line, start, i - start);
			if (!key)
				return (NULL);
			ft_printf("key: %s\n", key);
			val = get_var_val(envp, key);
			if (!val)
				return (free(key), NULL);
			ft_printf("value for %s is %s\n", key, val);
			// join key with preceding string (maybe take care of empty begin)
			tmp = ft_substr(line, 0, start - 1);
			if (!tmp)
				return (free(key), free(val), NULL);
			ret = ft_strjoin(tmp, val);
			free(tmp);
			free(key);
			free(val);
			if (!ret)
				return (NULL);
			remainder_line = ft_substr(line, i, ft_strlen(line));
			if (!remainder_line)
				return (free(ret), NULL);
			tmp = expand_variables(remainder_line, envp);
			free(remainder_line);
			if (!tmp)
				return (free(ret), NULL);
			new_ret = ft_strjoin(ret, tmp);
			if (!new_ret)
				return (NULL);
			// free(ret);
			// free(tmp);
			return (new_ret);
		}
		else if (line[i] == '$' && !is_valid_key(line[i + 1]))
			line[i] = ' ';
		i++;
	}
	return (line);
}
