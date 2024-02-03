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
char	**export_var(char **arr, const char *s)
{
	char	**tmp;
	char	*s_tmp;

	if (!arr || !s)
		return (NULL);
	int	index = find_key_env((const char **)arr, s, get_key_len);
	if (!arr)
		return (ft_printf("environment does not exist!\n"), NULL);
	if (index == -1)
	{
		tmp = append_str_arr((const char **)arr, s);
		if (!tmp)
			return (NULL);
		return (tmp);
	}
	s_tmp = ft_strdup(s);
	if (!s_tmp)
		return (ft_printf("error exporting variable\n"), NULL);
	free(arr[index]);
	arr[index] = s_tmp;
	return (arr);
}

static bool	check_valid_key(const char *s)
{
	int	i;

	i = 0;
	while (s[i] && s[i] != '=')
	{
		if (ft_isalnum(s[i]) || s[i] == '_')
			i++;
		else
			return (false);
	}
	return (true);
}

void	export(t_shell *shell)
{
	if (*shell->command && *(shell->command + 1)
		&& str_cchr(*(shell->command + 1), '=') == 1)
	{
		ft_printf("gets to export: %s\n", *(shell->command + 1));
		if (!check_valid_key(*(shell->command + 1)))
			return ((void)ft_printf("invalid variable name\n"));
		shell->tmp_arr = export_var(shell->owned_envp, *(shell->command + 1));
		if (!shell->tmp_arr)
			return ((void)ft_printf("add_env failed\n"));
		arr_free(shell->owned_envp);
		shell->owned_envp = shell->tmp_arr;
		shell->tmp_arr = NULL;
	}
	else
		ft_printf("export failed\n");
}
