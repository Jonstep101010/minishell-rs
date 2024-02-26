#include "expander.h"
#include "libft.h"
#include "environment.h"
#include "utils.h"

static void	skip_chars(const char *input, size_t *i, char c)
{
	while (input[*i] && input[*i] == c && input[*i + 1] && input[*i + 1] == c)
		(*i)++;
}

static void	handle_quotes(const char *input, t_expand *x)
{
	if (input[x->i] == '\'' && x->singlequote == 0)
		x->singlequote = 1;
	else if (input[x->i] == '\'' && x->singlequote == 1)
		x->singlequote = 0;
}

static void	set_end(const char *input, t_expand *x)
{
	x->end = x->i + 1;
	if (input[x->end] != '?')
		while (input[x->end]
			&& (ft_isalnum(input[x->end]) || input[x->end] == '_'))
			x->end++;
	else
		x->end++;
}

static char	*set_values(t_expand *x, char *input, const char **envp)
{
	x->to_expand = ft_substr(input, x->i, x->end - x->i);
	if (!x->to_expand)
		return (NULL);
	x->expanded = expand_var(x->to_expand, envp);
	free(x->to_expand);
	if (!x->expanded)
		return (NULL);
	x->before_expansion = ft_substr(input, 0, x->i);
	x->after_expansion = ft_strdup(&input[x->end]);
	x->tmp_len = ft_strlen(x->before_expansion) + ft_strlen(x->expanded) - 1;
	x->tmp = free_strjoin(2, x->before_expansion, x->expanded);
	free(input);
	input = ft_strjoin(x->tmp, x->after_expansion);
	free(x->after_expansion);
	free(x->tmp);
	if (!input)
		return (NULL);
	x->i = x->tmp_len + 1;
	return (input);
}

char	*expand_variables(t_expand *x, char *input, const char **envp)
{
	while (input && input[x->i] && input[x->i + 1])
	{
		skip_chars(input, &(x->i), '$');
		handle_quotes(input, x);
		if (!x->singlequote && input[x->i + 1] && input[x->i] == '$')
		{
			set_end(input, x);
			input = set_values(x, input, envp);
			if (!input)
				return (NULL);
		}
		else
			x->i++;
	}
	return (input);
}
