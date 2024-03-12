#include "libft.h"
#include <stdbool.h>
#include <stdio.h>
#include <sys/param.h>
#ifndef TEST
# include "expander.h"
#endif

// char    *expand_variables(t_expand *x, char *input, char *const *env);
char	*expand(char const *input, char *const *env);

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
 * @param env
 * @return char*
 */
char	*expander(char const *input_expander, char *const *env)
{
	t_expand	x;

	if (!input_expander || !env || !*env || !*input_expander)
		return (NULL);
	if (ft_strchr(input_expander, '$') == NULL)
		return (ft_strdup(input_expander));
	ft_bzero(&x, sizeof(t_expand));
	return (expand(input_expander, env));
}
