#include "tokens.h"
#include "utils.h"
#include "arr_utils.h"
#include <stdio.h>
#include "struct.h"
#include "commands.h"

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
			token[i].tmp_arr = append_str_arr((const char **)tmp,
					token[i].cmd_args[ii].elem);
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

void	add_pipes_as_tokens(t_shell *shell)
{
	size_t	i;
	size_t	len;

	shell->split_pipes = split_outside_quotes(shell->trimmed_line, "|");
	if (!shell->split_pipes)
		builtin_exit(shell, NULL);
	shell->tmp_arr = arr_trim(shell->split_pipes, WHITESPACE);
	if (!shell->tmp_arr)
		builtin_exit(shell, NULL);
	arr_free(shell->split_pipes);
	shell->split_pipes = shell->tmp_arr;
	len = arr_len((const char **)shell->split_pipes);
	shell->token = init_token(len);
	if (!shell->token)
		builtin_exit(shell, NULL);
	i = 0;
	while (i <= len)
	{
		shell->token[i].split_pipes = shell->split_pipes[i];
		i++;
	}
}
