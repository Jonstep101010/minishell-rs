#include "struct.h"
#include "libft.h"
#include "environment.h"

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

#include "tokens.h"

int	export(t_shell *shell, t_token *token)
{
	if (*token->command && token->command + 1 && *(token->command + 1)
		&& str_cchr(*(token->command + 1), '=') == 1 && arr_len((const char **)token->command) == 2)
	{
		ft_printf("gets to export: %s\n", *(token->command + 1));
		if (!check_valid_key(*(token->command + 1)))
			return ((void)ft_printf("invalid variable name\n"), -1);
		shell->tmp_arr = export_var(shell->owned_envp, *(token->command + 1));
		if (!shell->tmp_arr)
			return ((void)ft_printf("add_env failed\n"), -1);
		arr_free(shell->owned_envp);
		shell->owned_envp = shell->tmp_arr;
		shell->tmp_arr = NULL;
		return (0);
	}
	if (arr_len((const char **)token->command) > 2)
		ft_printf("Error: too many arguments!\n");
	else
		ft_printf("export failed\n");
	return (-1);
}
