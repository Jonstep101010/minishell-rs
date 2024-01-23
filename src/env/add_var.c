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

int	add_env(char **owned_envp, const char *s)
{
	char	**tmp;
	char	*s_tmp;

	ft_printf("from add_env:\n");
	ft_printf("%s\n", s);
	tmp = ft_split(s, '=');
	// [0] export VARNAME [1] value (in quotes or whatever)
	if (!tmp || !tmp[1] || !tmp[1][0])
		return (-1);
	s_tmp = tmp[1]; // should contain info about string
	print_arr((const char **)tmp);
	append_str_arr(owned_envp, s_tmp);
	// trim first string to remove before envar (ft_substr)
	ft_printf("added to env:\n");
	print_arr((const char **)owned_envp);
	return (0);
}

int	export(char **owned_envp, const char **cmd_arr)
{
	if (*cmd_arr && *(cmd_arr + 1) && str_cchr(*(cmd_arr + 1), '=') == 1)
	{
		ft_printf("gets to export\n");
		// if (*(cmd_arr + 1)[0] == '='
		// 	|| *(cmd_arr + 1)[ft_strlen(*(cmd_arr + 1))] == '=')
		// return (add_env(owned_envp, *(cmd_arr + 1)));
		add_env(owned_envp, *(cmd_arr + 1));
		ft_printf("no issues in export\n");
	}
	ft_printf("export failed\n");
	return (-1);
}
