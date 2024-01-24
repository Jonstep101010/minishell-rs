#include "ft_printf.h"
#include "libft.h"
#include "env.h"
#include <stdbool.h>

// export VARNAME="value"
// export VARNAME='value'
// export VARNAME=value
// split at '=' sign
// commands like these seem to go through lexer fine
// guaranteed to not have unbalanced quotes at this point

#include "utils.h"

#define NOT_FOUND -2
#define ERROR -1

int	find_key_env(char **owned_envp, const char **key_val)
{
	const char	*key;
	size_t	key_len;
	size_t	len;

	if (!key_val[0] || !key_val[1] || !key_val[1][0])
		return (ERROR);
	key = key_val[0];
	key_len = ft_strlen(key);
	len = null_arr_len(owned_envp);
	while (len--)
	{
		// if the keys match, return the index
		// should be replaced with new key
		if (ft_strncmp(owned_envp[len], key,
				key_len) == 0
			&& ft_strlen(owned_envp[len]) > key_len
				&& owned_envp[len][key_len + 1] == 	'=')
					return (len);

	}
	return (NOT_FOUND);
}

// key=value
int	add_env(char **owned_envp, const char *s)
{
	int		key_status;
	char	**key_val;

	// [0] KEY [1] value
	key_val = ft_split(s, '=');
	if (!key_val)
		return (-1);
	key_status = find_key_env(owned_envp, (const char **)key_val);
	// @todo check that there is a value for the key
	if (key_status == ERROR)
		return (-1);
	// replace
	if (key_status >= 0)
	{
		ft_printf("need to implement find and replace\n");
		return (0);
	}
	// add if not existing
	if (key_status == NOT_FOUND)
	{
		owned_envp = append_str_arr(owned_envp, s);
		ft_printf("added: %s\n", owned_envp[null_arr_len(owned_envp) - 1]);
		return (0);
	}
	return (-1);
}

int	export(char **owned_envp, const char **cmd_arr)
{

	if (*cmd_arr && *(cmd_arr + 1)
		&& str_cchr(*(cmd_arr + 1), '=') == 1)
	{
		ft_printf("gets to export\n");
		return (add_env(owned_envp, *(cmd_arr + 1)));
	}
	ft_printf("export failed\n");
	return (-1);
}
