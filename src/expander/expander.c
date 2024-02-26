#include "environment.h"
#include "libft.h"
#include <stdbool.h>
#include <stdio.h>
#include <sys/param.h>
#include "utils.h"

static bool	is_valid_key(int c)
{
	if (ft_isalnum(c) || c == '_')
		return (true);
	return (false);
}

static char	*expand_variables(char *input, const char **envp)
{
	char	*expanded;
	char	*after_expansion;
	char	*tmp;
	char	*to_expand;
	char	*before_expansion;
	size_t	i;
	size_t	tmp_len;
	size_t	end;
	int		singlequote;

	if (!input)
		return (NULL);
	i = 0;
	singlequote = 0;
	while (input && input[i] && input[i + 1])
	{
		while (input[i] && input[i] == '$' && input[i + 1] && input[i + 1] == '$')
			i++;
		if (input[i] == '\'' && singlequote == 0)
			singlequote = input[i];
		else if (input[i] == '\'' && singlequote == input[i])
			singlequote = 0;
		if (input[i] && input[i] == '$' && input[i + 1] && singlequote == 0)
		{
			end = i + 1;
			if (input[end] != '?')
				while (input[end] && is_valid_key(input[end]))
					end++;
			else
				end++;
			to_expand = ft_substr(input, i, end - i);
			if (!to_expand)
				return (NULL);
			expanded = expand_var(to_expand, envp);
			if (!expanded)
				return (free(to_expand), NULL);
			before_expansion = ft_substr(input, 0, i);
			after_expansion = ft_strdup(&input[end]);
			tmp_len = ft_strlen(before_expansion) + ft_strlen(expanded) - 1;
			free(to_expand);
			tmp = free_strjoin(2, before_expansion, expanded);
			free(input);
			input = ft_strjoin(tmp, after_expansion);
			free(after_expansion);
			if (!input)
				return (tmp);
			free(tmp);
			i = tmp_len + 1;
		}
		else
			i++;
	}
	return (input);
}

/**
 * @brief iterate over to find key
 * @details if key found, replace range of $key with value
 * \details if key contains $key, expand only its string to value
 * \details else iterate over to find next key
 * \details if there are $key$key_two, expand each key separately
 * \details if the expansion of $key results in key, do not expand
 * \details if the expansion of $key results in $key, expand
 * \details $invalid$key$keytwo -> valuevaluetwo (invalid empty)
 * @example input: $$USER -> $username
 * @param input_expander
 * @param envp
 * @return char*
 */
char	*expander(const char *input_expander, const char **envp)
{
	char	*restrict input;

	if (!input_expander || !envp || !*envp || !*input_expander)
		return (NULL);
	input = ft_strdup(input_expander);
	return (expand_variables(input, envp));
}
