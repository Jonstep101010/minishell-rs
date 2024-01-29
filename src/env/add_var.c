#include "ft_printf.h"
#include "libft.h"
#include "env.h"
#include <stdbool.h>

// export VARNAME="value"
// export VARNAME='value'
// export VARNAME=value
// guaranteed to not have unbalanced quotes at this point

#include "struct.h"
#include "utils.h"

// input like key=val
char	**add_env(char **arr, const char *s)
{
	char	**tmp;
	char	*s_tmp;

	if (!arr || !s)
		return (NULL);
	int	index = find_key_env(arr, s, get_key_len);
	if (!arr)
		return (ft_printf("error!\n"), NULL);
	if (index == -1)
	{
		tmp = append_str_arr(arr, s);
		if (!tmp)
			return (NULL);
		ft_printf("created new at end\n");
		return (tmp);
	}
	ft_printf("replacing %s with %s\n", arr[index], s);
	s_tmp = ft_strdup(s);
	if (!s_tmp)
	{
		ft_printf("error in var alloc!\n");
		exit(1);
	}
	free(arr[index]);
	arr[index] = s_tmp;
	ft_printf("new item: %s\n", arr[index]);
	return (arr);
}

void	export(t_shell *shell)
{
	if (*shell->command && *(shell->command + 1)
		&& str_cchr(*(shell->command + 1), '=') == 1)
	{
		ft_printf("gets to export: %s\n", *(shell->command + 1));
		shell->tmp_arr = add_env(shell->owned_envp, *(shell->command + 1));
		if (!shell->tmp_arr)
			return ((void)ft_printf("add_env failed\n"));
		shell->owned_envp = shell->tmp_arr;
		shell->tmp_arr = NULL;
	}
	else
		ft_printf("export failed\n");
}
