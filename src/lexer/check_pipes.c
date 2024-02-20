#include "lexer.h"
#include "minishell.h"
#include "libft.h"
#include "libutils.h"

bool	redir_valid(const char *s, const int redircount, char c)
{
	int	i;
	int	count;

	i = 0;
	count = redircount;
	if (count >= 1)
	{
		while (s[i])
		{
			while (s[i] && s[i] != c)
				i++;
			if (s[i] == c)
			{
				i++;
				if (s[i] == c)
					i++;
				count--;
			}
			while (s[i])
			{
				if (ft_isalnum(s[i]))
				{
					count++;
					i++;
					// printf("count: %d\n", count);
					break;
				}
				if (s[i] == '>' || s[i] == '<' || s[i] == '|')
					return (false);
				i++;
			}
		}
		if (count == redircount)
			return (true);
	}
	return (false);
}

bool	two_pipes_valid(const char *s, const int index)
{
	int	flag;
	int	i;

	i = 0;
	flag = 0;
	i = ft_strlen(s) - 1;
	flag = 0;
	while (s[i] && i >= index)
	{
		while_d(s, (int (*)(int))&isspace, 1, &i);
		flag = while_d(s, (int (*)(int))&ft_isalnum, 1, &i);
		while_d(s, (int (*)(int))&isspace, 1, &i);
		if (s[i] == '|' && flag == 1 && index + 1 == i)
			return (true);
		return (false);
	}
	return (false);
}

bool	pipes_valid(const char *s, const int pipes)
{
	int	i;
	int	count;

	i = 0;
	count = pipes;
	if (s[0] == '|')
		return (false);
	while (s[i])
	{
		if (while_not_i(s, (int (*)(int))&ft_isalnum,'|', &i) == 0)
			return (false);
		if (s[i] == '|' && s[i + 1] == '|' && pipes == 2)
			return (two_pipes_valid(s, i));
		while_i(s, (int (*)(int))isspace, 1, &i);
		count -= while_is_i(s, '|', &i);
		if (count > 2)
			return (false);
		while (s[i])
		{
			if (ft_isalnum(s[i]))
			{
				count++;
				i++;
				break;
			}
			if (s[i] == '>' || s[i] == '<' || s[i] == '|')
				break;
			i++;
		}
	}
	if (count == pipes)
		return (true);
	return (false);
}

t_lexer	check_pipes_redirection(const char *s, struct s_lexer *input)
{
	input->lexer = LEXER_PIPES;
	// something wrong with checking pipes @audit-info
	if (input->pipes > 0 && pipes_valid(s, input->pipes) == false)
		return (LEXER_PIPES);
	// check redirection
	input->lexer = LEXER_REDIRECTION;
	if (input->redir_greater > 0 && redir_valid(s, input->redir_greater, '>') == false)
		return (LEXER_REDIRECTION);
	if (input->redir_smaller > 0 && redir_valid(s, input->redir_smaller, '<') == false)
		return (LEXER_REDIRECTION);
	input->lexer = LEXER_SUCCESS;
	return (LEXER_SUCCESS);
}
