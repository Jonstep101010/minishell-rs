#include "lexer.h"
#include "minishell.h"
#include "libft.h"
#include "libutils.h"

static	bool	inner_loop(const char *s, int *i, int *count)
{
	while (s[*i])
	{
		if (ft_isalnum(s[*i]))
		{
			(*count)++;
			(*i)++;
			break ;
		}
		if (s[*i] == '>' || s[*i] == '<' || s[*i] == '|')
			return (false);
		(*i)++;
	}
	return (true);
}

static bool	redir_valid(const char *s, const int redircount, char c)
{
	int	i;
	int	count;

	i = 0;
	count = redircount;
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
		if (!inner_loop(s, &i, &count))
			return (false);
	}
	return (count == redircount);
}

static bool	two_pipes_valid(const char *s, const int index)
{
	int	flag;
	int	i;

	i = 0;
	flag = 0;
	i = ft_strlen(s) - 1;
	flag = 0;
	while (s[i] && i >= index)
	{
		while_d(s, ft_isspace, 1, &i);
		flag = while_d(s, &ft_isalnum, 1, &i);
		while_d(s, &ft_isspace, 1, &i);
		if (s[i] == '|' && flag == 1 && index + 1 == i)
			return (true);
		return (false);
	}
	return (false);
}

static bool	pipes_valid(const char *s, const int pipes)
{
	int	i;
	int	count;

	i = 0;
	count = pipes;
	while (s[i])
	{
		if (while_not_i(s, &ft_isalnum, '|', &i) == 0)
			return (false);
		if (s[i] == '|' && s[i + 1] == '|' && pipes == 2)
			return (two_pipes_valid(s, i));
		while_i(s, ft_isspace, 1, &i);
		count -= while_is_i(s, '|', &i);
		while (s[i])
		{
			if (ft_isalnum(s[i]))
				return (count + 1 == pipes);
			if (s[i] == '>' || s[i] == '<' || s[i] == '|')
				break ;
			i++;
		}
	}
	return (count == pipes);
}

t_lexer	check_pipes_redirection(const char *s, struct s_lexer *input)
{
	if (input->pipes > 0)
	{
		input->lexer = LEXER_PIPES;
		if (s[0] == '|' || s[ft_strlen(s) - 1] == '|')
			return (LEXER_PIPES);
		if (!pipes_valid(s, input->pipes))
			return (LEXER_PIPES);
	}
	input->lexer = LEXER_REDIRECTION;
	if (input->redir_greater > 0 && !redir_valid(s, input->redir_greater, '>'))
		return (LEXER_REDIRECTION);
	if (input->redir_smaller > 0 && !redir_valid(s, input->redir_smaller, '<'))
		return (LEXER_REDIRECTION);
	input->lexer = LEXER_SUCCESS;
	return (LEXER_SUCCESS);
}
