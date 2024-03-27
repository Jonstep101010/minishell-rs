#include "libft.h"
#include <stddef.h>
#include "utils.h"
#include "arr_utils.h"

void	rm_str_arr(char **arr, const char *s)
{
	size_t	i;
	size_t	len;

	if (!arr || !s)
		return ;
	i = 0;
	len = arr_len(arr);
	while (arr[i])
	{
		if (equal(arr[i], s))
		{
			free(arr[i]);
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

char	**arr_trim(char **arr, char const *set)
{
	size_t	i;
	size_t	len;
	char	**ret;

	if (!arr)
		return (NULL);
	if (!set)
		return (arr);
	i = 0;
	len = arr_len(arr);
	ret = malloc(sizeof(char *) * (len + 1));
	if (!ret)
		return (NULL);
	while (arr[i])
	{
		ret[i] = ft_strtrim(arr[i], set);
		i++;
	}
	ret[i] = NULL;
	return (ret);
}

int	arr_ncmp(char *const *arr1, char *const *arr2, size_t n)
{
	size_t	i;

	i = 0;
	if (!arr1 || !arr2)
		return (1);
	while (n-- > 0)
	{
		if (ft_strlen(arr1[i]) != ft_strlen(arr2[i])
			|| ft_strncmp(arr1[i], arr2[i], ft_strlen(arr1[i])) != 0)
				return (1);
		i++;
	}
	if (arr1[i] || arr2[i])
		return (1);
	return (0);
}
