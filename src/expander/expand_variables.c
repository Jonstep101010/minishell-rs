#include "libft.h"
#include <stdbool.h>

typedef struct s_expander
{
	size_t	i;
	char	*ret;
	char	*key;
	size_t	start;
	char	*val;
	char	*tmp;
	int		singlequote;
	char	*remainder_line;
	char	*line;
}	t_expander;


static bool	is_valid_key(int c)
{
	if (ft_isalnum(c) || c == '_')
		return (true);
	return (false);
}

char	*get_var_val(const char **arr, const char *key);
char	*expand_variables(const char *input, const char **envp);

static char *replacer(t_expander *x, const char **envp)
{
	if (!x->ret)
		return (NULL);
	if (x->i <= ft_strlen(x->line))
	{
		x->remainder_line = ft_strdup(&x->line[x->i]);
		free(x->line);
		if (!x->remainder_line)
			return (x->ret);
		// printf("remainder: %s\n", x->remainder_line);
		x->tmp = ft_strjoin(x->ret, x->remainder_line);
		free(x->remainder_line);
		free(x->ret);
		x->ret = expand_variables(x->tmp, envp);
		free(x->tmp);
	}
	return (x->ret);
}

static char	*insert_value(t_expander *x, const char **envp)
{
	x->key = ft_substr(x->line, x->start, x->i - x->start);
	if (!x->key)
		return (NULL);
	x->val = get_var_val(envp, x->key);
	free(x->key);
	if (!x->val)
		return (NULL);
	x->tmp = ft_substr(x->line, 0, x->start - 1);
	if (!x->tmp)
		return (free(x->val), NULL);
	x->ret = ft_strjoin(x->tmp, x->val);
	free(x->tmp);
	free(x->val);
	return (x->ret);
}

static char	*check_line(t_expander *x, const char **envp)
{
	while (x->line && x->line[x->i])
	{
		if (x->line[x->i] == '\'' && x->singlequote == 0)
			x->singlequote = x->line[x->i];
		else if (x->line[x->i] == '\'' && x->singlequote == x->line[x->i])
			x->singlequote = 0;
		else if (x->line[x->i] && x->line[x->i + 1] && x->line[x->i] == '$'
			&& x->singlequote == 0 && (is_valid_key(x->line[x->i + 1]) || x->line[x->i + 1] == '?'))
		{
			x->i++;
			x->start = x->i;
			if (x->line[x->i] != '?')
				while (x->line[x->i] && is_valid_key(x->line[x->i]))
					x->i++;
			else
				x->i++;
			if (x->start == x->i)
				continue;
			if (!insert_value(x, envp))
				return (NULL);
			return (replacer(x, envp));
		}
		if (x->line[x->i] == '$' && x->singlequote == 0 && x->line[x->i + 1])
			x->line[x->i] = ' ';
		x->i++;
	}
	return (x->line);
}

// @follow-up handle in parentheses
/* @todo correct behaviour with quotes (pass in from top, same in recursion (shared state))*/
char	*expand_variables(const char *input, const char **envp)
{
	t_expander	x;

	x.i = 0;
	x.singlequote = 0;
	x.line = ft_strdup(input);
	return (check_line(&x, envp));
}
