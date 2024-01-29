#include "ft_printf.h"
#include "libft.h"
#include <stddef.h>
#include "utils.h"

/**
 * @brief returns copy with new item added, frees old
 *
 * @param arr heap-allocated 2d array
 * @param s possibly stack allocated string
 * @return char** copy with new element
 */
char	**append_str_arr(char **arr, const char *s)
{
	size_t	len;
	size_t	i;
	char	**ret;

	if (!s || !*s)
		return (NULL);
	len = 0;
	if (arr)
		len = arr_len((const char **)arr);
	ret = (char **) ft_calloc(len + 2, sizeof(char *));
	if (!ret)
		return (NULL);
	i = 0;
	while (arr && arr[i])
	{
		ret[i] = arr[i];
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
	ft_printf("%zu(len)", len + 1);
	print_arr_sep(ret, '{', '}');
	free(arr);
	return (ret);
}

// @follow-up @audit-info this can only find the exact string, might need something that enables find and replace
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
