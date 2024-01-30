#include "env.h"
#include "parser.h"
#include "libft.h"
#include <stdbool.h>

static bool	is_valid_key(int c)
{
	if (ft_isalnum(c) || c == '_')
		return (true);
	return (false);
}

static void	handle_singlequotes(t_expander *ex, const char *line)
{
	if (line[ex->i] == '\'' && ex->singlequote == 0)
		ex->singlequote = line[ex->i];
	else if (line[ex->i] == '\'' && ex->singlequote == line[ex->i])
		ex->singlequote = 0;
}

static char	*replace_variables(t_expander *ex, const char *line, const char **envp)
{
	ex->key = ft_substr(line, ex->start, ex->i - ex->start);
	if (!ex->key)
		return (NULL);
	ex->val = get_var_val(envp, ex->key);
	free(ex->key);
	if (!ex->val)
		return (NULL);
	ex->tmp = ft_substr(line, 0, ex->start - 1);
	if (!ex->tmp)
		return (free(ex->val), NULL);
	ex->ret = ft_strjoin(ex->tmp, ex->val);
	free(ex->val);
	free(ex->tmp);
	if (!ex->ret)
		return (NULL);
	ex->remainder_line = ft_substr(line, ex->i, ft_strlen(line));
	if (!ex->remainder_line)
		return (NULL);
	ex->tmp = expand_variables(ex->remainder_line, envp);
	if (!ex->tmp)
		return (free(ex->remainder_line), NULL);
	ex->new_ret = ft_strjoin(ex->ret, ex->tmp);
	free(ex->ret);
	free(ex->tmp);
	return (ex->new_ret);
}

/**
 * @brief expands and replaces environment variables, non needed '$' will be filled with spaces
 * will replace valid keys that are not found with empty strings
 *
 * @param line raw input line
 * @param envp envars
 * @return char*
 */
char	*expand_variables(char *line, const char **envp)
{
	t_expander	ex;

	ex.i = 0;
	ex.singlequote = 0;
	while (line[ex.i])
	{
		handle_singlequotes(&ex, line);
		if (line[ex.i] && line[ex.i + 1] && line[ex.i] == '$'
			&& ex.singlequote == 0 && is_valid_key(line[ex.i + 1]))
		{
			ex.i++;
			ex.start = ex.i;
			while (line[ex.i] && is_valid_key(line[ex.i]))
				ex.i++;
			if (ex.start == ex.i)
				continue;
			return (replace_variables(&ex, line, envp));
		}
		if (line[ex.i] == '$' && ex.singlequote == 0)
			line[ex.i] = ' ';
		ex.i++;
	}
	return (line);
}
