#include <stddef.h>
#include <sys/param.h>
#include "arr_utils.h"
#include "libft.h"
#include "tokens.h"
#include "utils.h"
#include "libutils.h"
#include "struct.h"
#include "builtins.h"
#include "parser.h"

void	set_cmd_func(t_token *token);

t_arg	*init_cmdargs(size_t size)
{
	t_arg	*args;

	args = ft_calloc(sizeof(t_arg), (size + 1));
	if (!args)
		return (NULL);
	return (args);
}

t_token	*init_token(size_t size)
{
	t_token	*token;

	token = ft_calloc(sizeof(t_token), (size + 1));
	if (!token)
		return (NULL);
	token[size].cmd_args = NULL;
	token[size].tmp_arr = NULL;
	token[size].command = NULL;
	return (token);
}


void	add_pipe_split_as_tokens(char **pipe_split, t_shell *shell)
{
	size_t	i;
	size_t	len;

	if (!pipe_split)
		return ;
	len = arr_len((const char **)pipe_split);

	i = 0;
	shell->token = init_token(len);
	if (!shell->token)
		return ;
	while (i <= len)
	{
		shell->token[i].split_pipes = pipe_split[i];
		i++;
	}
}

char	*expander(const char *input, const char **envp);

// take the token with command string and split it into command and arguments
// if we find any pipes
void	convert_split_token_string_array_to_tokens(t_shell *shell)
{
	size_t	i;
	size_t	ii;
	size_t	len;

	i = 0;
	ii = 0;
	char	*tmp;
	if (!shell->token || !shell->token[i].split_pipes)
		return ;
	while (shell->token[i].split_pipes)
	{
		// split into command and arguments
		shell->token[i].tmp_arr = split_outside_quotes(shell->token[i].split_pipes, WHITESPACE);
		if (!shell->token[i].tmp_arr)
			return ;
		len = arr_len((const char **)shell->token[i].tmp_arr);
		if (len == 0)
			return ;
		// create cmd_args with space for the arguments
		shell->token[i].cmd_args = init_cmdargs(len);
		if (!shell->token[i].cmd_args)
			return ;
		ii = 0;
		// printf("elem: %s, %d\n", shell->token[i].cmd_args[0].elem, shell->token[i].builtin_info);
		while (shell->token[i].tmp_arr[ii])
		{
			// store the cmd_args in the token
			shell->token[i].cmd_args[ii].elem = shell->token[i].tmp_arr[ii];
			if (!shell->token[i].cmd_args[ii].elem)
				return ;
			set_cmd_func(&shell->token[i]);
			// @follow-up add more properties (count?), separate function?
			if (str_cchr(shell->token[i].cmd_args[ii].elem, '\'') == 0 && str_cchr(shell->token[i].cmd_args[ii].elem, '"') == 0)
				shell->token[i].cmd_args[ii].quote = NONE;
			if (str_cchr(shell->token[i].cmd_args[ii].elem, '"'))
				shell->token[i].cmd_args[ii].quote = DOUBLE;
			if (str_cchr(shell->token[i].cmd_args[ii].elem, '\''))
				shell->token[i].cmd_args[ii].quote = SINGLE;
			tmp = expander(shell->token[i].cmd_args[ii].elem, (const char **)shell->owned_envp);
			// fprintf(stderr, "tmp in convert: %s\n", tmp);
			if (!tmp)
				return ;
			if (ft_strncmp(tmp, shell->token[i].cmd_args[ii].elem, MAX(ft_strlen(tmp), ft_strlen(shell->token[i].cmd_args[ii].elem))== 0))
			{
				// printf("recursive expansion is the same as the original, freeing\n");
				// printf("reex: %s\n", shell->token[i].cmd_args[ii].elem);
				free(tmp);
				break ;
			}
			free(shell->token[i].cmd_args[ii].elem);
			shell->token[i].cmd_args[ii].elem = tmp;
			int quote = 0;
			if (shell->token[i].cmd_args[ii].quote != NONE)
			{
				tmp = do_quote_bs(shell->token[i].cmd_args[ii].elem, &quote);
				if (!tmp)
					return ;
				free(shell->token[i].cmd_args[ii].elem);
				shell->token[i].cmd_args[ii].elem = tmp;
			}
			shell->token[i].cmd_args[ii].type = STRING; // default type
			ii++;
		}
		free(shell->token[i].tmp_arr);
		i++;
	}
}
