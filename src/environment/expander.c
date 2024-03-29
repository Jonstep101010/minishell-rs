/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expander.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:32:25 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:36:33 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"
#include "str_utils.h"
#include "utils.h"
#include "environment.h"

static int	check_index_advance(const char *s, int i)
{
	int	count;

	count = 0;
	while (s[i + count] && s[i + count + 1]
		&& !ft_strchr("$\"'/? )(", s[i + count + 1]))
		count++;
	if (s[i + count] && s[i + count + 1] == '?')
		count += 1;
	return (count);
}

static char	*expand_inside(char *key, char *const *env, int *i)
{
	const size_t	len = ft_strlen(key);
	char			*ret;

	ret = NULL;
	if (*key)
		ret = get_env(env, key);
	if (!ret)
		ret = ft_strdup("");
	free(key);
	if (!ret)
		return (NULL);
	*i += len;
	return (ret);
}

/**
 * @brief checks status of quotes at current position
 * @param expand true if allowed to expand
 */
static void	check_quotes(const char *s, bool *expand, int *double_quote)
{
	if (*s == '"' && *double_quote == 0)
		*double_quote = 1;
	else if (*s == '"' && *double_quote == 1)
		*double_quote = 0;
	if (*s == '\'' && *double_quote == 0
		&& s[check_index_advance(s, 0)] != '"')
		*expand = !*expand;
}

/**
 * @brief expands variables in string
 * @details accounts for edge cases such as
 * \details "'$VAR'" ('VAL') "'"'$VAR'"'" ('$VAR')
 * @param s
 * @param env
 * @return char*
 */
static char	*expand(char const *s, char *const *env)
{
	int		i;
	bool	expand;
	int		double_quote;
	char	*ret;
	char	*key;

	i = -1;
	expand = true;
	double_quote = 0;
	ret = ft_strdup("");
	while (s[++i])
	{
		check_quotes(&s[i], &expand, &double_quote);
		if (expand && (s[i] == '$'
				&& s[i + 1] && !ft_strchr("$()", s[i + 1])))
		{
			key = ft_substr(s, i + 1, check_index_advance(s, i));
			if (!key)
				return (free(ret), eprint("alloc fail!"), NULL);
			ret = free_both_join(ret, expand_inside(key, env, &i));
		}
		else
			ret = append_char_str(ret, s[i]);
	}
	return (ret);
}

/**
 * @brief iterate over to find key
 * @details if key found, replace range of $key with value
 * \details if key contains $key, expand only its string to value
 * \details else iterate over to find next key
 * \details if there are $key$key_two, expand each key separately
 * \details if the expansion of $key results in key, do not expand
 * \details $invalid$key$keytwo -> valuevaluetwo (invalid empty)
 * @example input: $$USER -> $username
 * @param input_expander
 * @param env
 * @return char*
 */
char	*expander(char const *input_expander, char *const *env)
{
	if (!input_expander || !env || !*env || !*input_expander)
		return (NULL);
	if (ft_strchr(input_expander, '$') == NULL)
		return (ft_strdup(input_expander));
	return (expand(input_expander, env));
}
