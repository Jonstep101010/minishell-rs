#include <stddef.h>
#include "tokens.h"
#include "libft.h"
#include "utils.h"

void	convert_tokens_to_string_array(t_token *token)
{
	size_t	i;
	size_t	ii;
	char	**tmp;

	i = 0;
	ii = 0;
	if (!token || !token->cmd_args || !token->cmd_args[0].elem)
		return ;
	while (token[i].cmd_args)
	{
		ii = 0;
		tmp = NULL;
		while (token[i].cmd_args[ii].elem)
		{
			token[i].tmp_arr = append_str_arr((const char **)tmp, token[i].cmd_args[ii].elem);
			arr_free(tmp);
			if (!token[i].tmp_arr)
				return ((void)printf("something went to shit!\n"));
			tmp = token[i].tmp_arr;
			ii++;
		}
		token[i].command = token[i].tmp_arr;
		i++;
	}
}
