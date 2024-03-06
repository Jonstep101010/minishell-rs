#include <stdio.h>
#include "arr_utils.h"

void	print_arr_sep(char **arr, char sep_open, char sep_close)
{
	size_t	i;
	size_t	len;

	i = 0;
	if (!arr)
		return;
	len = arr_len(arr);
	if (!arr)
		return ;
	while (i <= len)
	{
		printf("[%lu]%c%s%c", i, sep_open, arr[i], sep_close);
		i++;
	}
	printf("\n");
}
