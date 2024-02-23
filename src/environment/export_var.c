#include "libft.h"
#include "environment.h"
#include <stdbool.h>

// export VARNAME="value"
// export VARNAME='value'
// export VARNAME=value
// guaranteed to not have unbalanced quotes at this point

#include "utils.h"

// search for key
// make it key=value if it exists
void	update_variable(char **envp, const char *key, const char *value)
{
	char	*key_value;
	char	*tmp;
	int		index;

	index = find_key_env((const char **)envp, key, ft_strlen);
	if (index == -1)
		return ;// error handling?
	tmp = ft_strjoin(key, "=");
	if (!tmp)
		return ;
	key_value = free_first_join(tmp, value);
	if (!key_value)
		return ;
	free(envp[index]);
	envp[index] = key_value;
}

// input like key=val
char	**export_var(char **arr, const char *s)
{
	char	**tmp;
	char	*s_tmp;

	if (!arr || !s)
		return (NULL);
	int	index = find_key_env((const char **)arr, s, get_key_len);
	if (!arr)
		return (NULL);
	if (index == -1)
	{
		tmp = append_str_arr((const char **)arr, s);
		if (!tmp)
			return (NULL);
		return (tmp);
	}
	s_tmp = ft_strdup(s);
	if (!s_tmp)
		return (NULL);
	free(arr[index]);
	arr[index] = s_tmp;
	return (arr);
}
