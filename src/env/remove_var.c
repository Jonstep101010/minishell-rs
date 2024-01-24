#include "env.h"
#include "libft.h"
#include "utils.h"

// see if exists, remove using rm_str_arr
// otherwise: error with not found
char	**rm_env(char **arr, const char *s)
{
	if (!arr || !s)
		return (NULL);
	int	index = find_key_env(arr, s, ft_strlen);
	if (!arr)
		return (ft_printf("error!\n"), NULL);
	if (index == -1)
	{
		ft_printf("Error: varname does not exist in env!\n");
	}
	else if (index >= 0 && arr[index])
	{
		ft_printf("removing variable: %s\n", arr[index]);
		rm_str_arr(arr, arr[index]);
		if (!arr)
			return (NULL);
	}
	return (arr);
}


