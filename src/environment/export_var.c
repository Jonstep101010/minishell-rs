#include "libft.h"
#include "environment.h"
#include <stdbool.h>

// export VARNAME="value"
// export VARNAME='value'
// export VARNAME=value
// guaranteed to not have unbalanced quotes at this point

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
		return (printf("environment does not exist!\n"), NULL);
	if (index == -1)
	{
		tmp = append_str_arr((const char **)arr, s);
		if (!tmp)
			return (NULL);
		return (tmp);
	}
	s_tmp = ft_strdup(s);
	if (!s_tmp)
		return (printf("error exporting variable\n"), NULL);
	free(arr[index]);
	arr[index] = s_tmp;
	return (arr);
}
