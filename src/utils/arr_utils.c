#include "libft.h"
#include <stddef.h>
#include "utils.h"

size_t	null_arr_len(char **arr)
{
	if (!arr)
		return (0);
	return (arr_len((const char **)arr));
}

char	**append_str_arr(char **arr, const char *s)
{
	size_t	len;
	size_t	i;
	char	**ret;

	if (!s || !*s)
		return (NULL);
	len = null_arr_len(arr);
	ret = (char **) ft_calloc(len + 2, sizeof(char *));
	if (!ret)
		return (NULL);
	i = 0;
	while (arr && arr[i] && i <= len)
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

void	rm_str_arr(char **arr, const char *s)
{
	size_t	i;
	size_t	len;

	if (!arr || !s)
		return ;
	i = 0;
	len = arr_len((const char **)arr);
	while (arr[i])
	{
		if (occurs_exclusively(arr[i], s))
		{
			free_null(arr[i]);
			while (i < len)
			{
				arr[i] = arr[i + 1];
				i++;
			}
			return;
		}
		i++;
	}
}
