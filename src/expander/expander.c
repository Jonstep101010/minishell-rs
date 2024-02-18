#include "libft.h"
#include <stdbool.h>
#include <stdio.h>
#include <sys/param.h>
#include "environment.h"

char	*expand_variables(const char *input, const char **envp);
char	*expand_in_string(const char *input, const char **envp);
char	*guard_expander(const char *input, const char **envp)
{
	if (!input || !envp || !*envp || !*input)
		return (NULL);
	return (expand_variables(input, envp));
}

char	*join_strings_free(int count, ...);

char	*expand_variables(const char *input, const char **envp)
{
	size_t	i;
	size_t	j;
	size_t	len;
	char	*expanded;
	char	*key;
	char	*val;
	char	*tmp;

	i = 0;
	len = ft_strlen(input);
	expanded = ft_strdup(input);
	if (!expanded)
		return (NULL);
	while (i < len)
	{
		while (expanded[i] == '$' && expanded[i + 1] == '$')
			i++;
		if (expanded[i] == '$')
		{
			j = i + 1;
			while (j < len && ft_isalnum(expanded[j]))
				j++;
			key = ft_substr(expanded, i + 1, j - i - 1);
			if (!key)
				return (free(expanded), NULL);
			val = expand_in_string(key, envp);
			free(key);
			if (!val)
				return (free(expanded), NULL);
			tmp = ft_strjoin(expanded, val);
			free(val);
			free(expanded);
			if (!tmp)
				return (NULL);
			expanded = tmp;
			len = ft_strlen(expanded);
		}
		i++;
	}
	return (expanded);
}



// iterate over to find key

// if key found, replace range of $key with value

// if key contains $key, expand only its string to value

// else iterate over to find next key

// if there are $key$key_two, expand each key separately

// if the expansion of $key results in key, do not expand

// if the expansion of $key results in $key, expand

// an input string like $invalid$key$keytwo will result in valuevaluetwo (invalid is replaced with empty)
// $$key$keytwo will result in $valuevaluetwo
char	*expander(const char *input_expand, const char **envp)
{
	char	*expanded;
	char	*tmp2;
	char	*after_expansion;
	char	*tmp;
	char	*to_expand;
	char	*before_expansion;
	size_t	i;
	size_t	tmp_len;
	size_t	end;

	if (!input_expand || !envp || !*envp)
		return (NULL);

	char	*input = ft_strdup(input_expand);
	if (!input)
		return (NULL);
	if (ft_strlen(input) == 1)
		return (input);
	// source string with $$USER
	i = 0;// @todo implement quotes
	while (input[i])
	{
		while (input[i] && input[i] == '$' && input[i + 1] == '$')
			i++;
		if (input[i] && input[i] == '$')// if variable denotation
		{
			end = i + 1;
			while (input[end] && (ft_isalnum(input[end]) || input[end] == '_'))
				end++;
			fprintf(stderr, "%s\n", &input[end]);
			to_expand = ft_substr(input, i, end - i);
			printf("to_expand: %s\n", to_expand);
			if (!to_expand)
				return (NULL);
			expanded = expand_in_string(to_expand, envp);
			printf("expanded: %s\n", expanded);
			if (!expanded)
				return (free(to_expand), NULL);
			before_expansion = ft_substr(input, 0, i);
			after_expansion = ft_strdup(&input[end]);
			fprintf(stderr, "before_expansion: %s\n", before_expansion);
			fprintf(stderr, "expanded: %s\n", expanded);
			tmp_len = ft_strlen(before_expansion) + ft_strlen(expanded) - 1;
			free(to_expand);
			if (tmp_len <= 0)
				return (free(expanded), NULL);
			tmp = ft_strjoin(before_expansion, expanded);
			fprintf(stderr, "tmp: %s\n", tmp);
			free(expanded);
			free(before_expansion);
			fprintf(stderr, "after_expansion: %s\n", after_expansion);
			tmp2 = ft_strjoin(tmp, after_expansion);
			free(after_expansion);
			if (!tmp2)
				return (free(input), tmp);
			free(tmp);
			fprintf(stderr, "tmp2: %s\n", tmp2);
			free(input);
			input = tmp2;
			fprintf(stderr, "internal_input: %s\n", input);
			i = tmp_len - 1;
		}
		else
			i++;
	}
	return (input);
}
