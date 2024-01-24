#include "ft_printf.h"
#include "libft.h"
#include "env.h"
#include <stdbool.h>

// export VARNAME="value"
// export VARNAME='value'
// export VARNAME=value
// guaranteed to not have unbalanced quotes at this point

#include "utils.h"

// input like key=val
char	**add_env(char **arr, const char *s)
{
	char	**tmp;
	char	*s_tmp;

	if (!arr || !s)
		return (NULL);
	int	index = find_key_env(arr, s, get_key_len);
	if (!arr || get_key_len(s) < 0)
		return (ft_printf("error!\n"), NULL);
	if (index == -1)
	{
		tmp = append_str_arr(arr, s);
		if (!tmp)
			return (NULL);
		free_null(arr);
		arr = tmp;
		ft_printf("created new at end: %s\n", arr[null_arr_len(arr) - 1]);
	}
	else if (index >= 0 && arr[index])
	{
		ft_printf("replacing %s with %s\n", arr[index], s);
		s_tmp = ft_strdup(s);
		if (!s_tmp)
			return (NULL);
		free_null(arr[index]);
		arr[index] = s_tmp;
	}
	return (arr);
}

int	export(char **owned_envp, const char **cmd_arr)
{
	char	**tmp;

	if (*cmd_arr && *(cmd_arr + 1)
		&& str_cchr(*(cmd_arr + 1), '=') == 1)
	{
		ft_printf("gets to export\n");
		tmp = add_env(owned_envp, *(cmd_arr + 1));
		if (!tmp)
			return (ft_printf("export failed\n"), -1);
		return (0);
	}
	ft_printf("export failed\n");
	return (-1);
}
