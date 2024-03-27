#include "struct.h"
#include "tokens.h"
#include "libft.h"
#include "libutils.h"

#ifndef TEST_TOKENS_H
# include "libft.h"
# include <stdlib.h>
# include "arr_utils.h"
#endif

void	destroy_all_tokens(t_shell *shell)
{
	t_token	*token;
	size_t	i;
	size_t	ii;

	if (!shell->token || !(shell->token->split_pipes))
		return ;
	token = shell->token;
	i = 0;
	while (token[i].split_pipes)
	{
		if (token[i].cmd_args)
		{
			ii = 0;
			while (token[i].cmd_args[ii].elem)
			{
				free_null(&token[i].cmd_args[ii].elem);
				ii++;
			}
			free_null(&token[i].cmd_args);
		}
		free_null(&(token[i].split_pipes));
		if (token[i].bin)
			free_null(&(token[i].bin));
		i++;
	}
	free_null(&shell->token);
}
