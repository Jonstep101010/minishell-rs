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
		// printf("gets to export: %s\n", *(token->command + 1));
		if (!check_valid_key(*(token->command + 1)))
			return (1);
		// (void)printf("invalid variable name\n")
		shell->tmp_arr = export_var(shell->owned_envp, *(token->command + 1));
		if (!shell->tmp_arr)
			return (0);
		if (shell->owned_envp != shell->tmp_arr)
			arr_free(shell->owned_envp);
		shell->owned_envp = shell->tmp_arr;
		shell->tmp_arr = NULL;
		return (0);
	}
	// if (arr_len((const char **)token->command) > 2)
	// 	printf("Error: too many arguments!\n");
	// else
	// 	printf("export failed\n");
	return (1);
}
