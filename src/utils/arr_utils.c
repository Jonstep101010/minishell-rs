#include "ft_printf.h"
#include "libft.h"
#include <stddef.h>
#include "utils.h"

char	**append_str_arr(const char **arr, const char *s)
{
	size_t	len;
	size_t	i;
	char	**ret;

	if (!s || !*s)
		return (NULL);
	len = arr_len((const char **)arr);
	ret = (char **) ft_calloc(len + 2, sizeof(char *));
	if (!ret)
		return (NULL);// leave original array intact
	i = 0;
	while (arr && arr[i] && i <= len)
	{
		ret[i] = ft_strdup(arr[i]);// duplicate instead
		if (!ret[i])
		{
			arr_free(ret);
			return (NULL);
		}
		i++;
	}
	ret[i] = ft_strdup(s);
	if (!ret[i])
		return (arr_free(ret), NULL);
	ft_printf("%zu(len)", len + 2);
	print_arr_sep(ret, '{', '}');
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
