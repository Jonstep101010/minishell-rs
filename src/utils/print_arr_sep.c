#include "stddef.h"
#include "libft.h"

void	print_arr_sep(char **arr, char sep_open, char sep_close)
{
	size_t	i;
	size_t	len;

	i = 0;
	if (!arr)
		return;
	len = arr_len((const char **)arr);
	if (!arr)
		return ;
	while (i <= len)
	{
		ft_printf("[%d]%c%s%c", i, sep_open, arr[i], sep_close);
		i++;
	}
	ft_printf("\n");
}
