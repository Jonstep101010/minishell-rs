#include "libft.h"
#include <stddef.h>

char	**append_2d_arr(char **arr, const char *s)
{
	size_t	len;
	size_t	i;
	char	**ret;

	if (!arr || !*arr || !s || !*s)
		return (NULL);
	len = arr_len((const char **)arr);
	ret = (char **) ft_calloc(len + 2, sizeof(char *));
	if (!ret)
		return (NULL);
	i = 0;
	while (arr[i] && i < len)
	{
		ret[i] = ft_strdup(arr[i]);
		if (!ret[i])
		{
			arr_free(ret);
			return (NULL);
		}
		i++;
	}
	ret[i] = ft_strdup(s);
	if (!ret[i])
		return (NULL);
	return (ret);
}
