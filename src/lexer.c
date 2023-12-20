#include "libft.h"
#include "minishell.h"
#include "struct.h"
#include <stdbool.h>

static int	str_cchr(const char *s, char c)
{
	int	i;
	int	count;

	i = 0;
	count = 0;
	while (s[i])
	{
		if (s[i] == c)
			count++;
		i++;
	}
	return (count);
}

void	count_number(char *s, struct s_lexer *input)
{
	*input = (struct s_lexer){
		.singlequotes = str_cchr(s, '\''),
		.doublequotes = str_cchr(s, '\"'),
		.open_curly_brackets = str_cchr(s, '{'),
		.close_curly_brackets = str_cchr(s, '}'),
		.open_square_brackets = str_cchr(s, '['),
		.close_square_brackets = str_cchr(s, ']'),
		.open_parentheses = str_cchr(s, '('),
		.close_parentheses = str_cchr(s, ')'),
		.redir_greater = str_cchr(s, '>'),
		.redir_smaller = str_cchr(s, '<'),
		.pipes = str_cchr(s, '|'),
	};
}

// never allowed: "> |"
// handle input like "> > >" "> >" "ls > > outfile" "ls >"
// handle input like "ls > outfile >" "ls > outfile > >" "ls > outfile > > >"
// valid "> outfile" (creates outfile) "ls > outfile" (redirects to outfile)
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
		while (s[i] && ft_isspace(s[i]) == 1)
			i--;
		// while_string_wrapper(s, ft_isspace, 1, &i)
		while (s[i] && ft_isalnum(s[i]) == 1)
		{
			flag = 1;
			i--;
		}
		while (s[i] && ft_isspace(s[i]) == 1)
			i--;
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
	int	flag;

	i = 0;
	flag = 0;
	count = pipes;
	if (count >= 1)
	{
		if (s[0] == '|')
			return (false);
		while (s[i])
		{
			flag = 0;
			while (s[i] && s[i] != '|')
			{
				if (ft_isalnum(s[i]) == 1)
					flag = 1;
				i++;
			}
			if (flag == 0)
				return (false);
			if (s[i] == '|' && s[i + 1] == '|' && pipes == 2)
				return (two_pipes_valid(s, i));
			if (s[i] && s[i] == '|')
			{
				i++;
				while (s[i] && ft_isspace(s[i]))
					i++;
				if (s[i] == '|' && s[i + 1] != '|')
				{
					i++;
					count--;
				}
				else if ((s[i] == '|' && s[i + 1] == '|'))
					return (false);
				count--;
			}
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
	}
	return (false);
}

// make return value an enum
enum e_lexer	lexer(char *s)
{
	if (!s || !*s)
		return (LEXER_NULL);
	// char *s = ft_strtrim(str, " ");
	struct s_lexer	input;
	count_number(s, &input);

	// check pipes
	if (input.pipes > 0)
	{
		if (pipes_valid(s, input.pipes) == false)
			return (LEXER_PIPES);
		// only for testing purposes
		return (LEXER_SUCCESS);
	}
	// check redirection
	if (input.redir_greater > 0)
	{
		if (redir_valid(s, input.redir_greater, '>') == false)
			return (LEXER_REDIRECTION);
	}
	if (input.redir_smaller > 0)
	{
		if (redir_valid(s, input.redir_smaller, '<') == false)
			return (LEXER_REDIRECTION);
	}
	if (input.singlequotes % 2 != 0 || input.doublequotes % 2 != 0)
		return (LEXER_UNBALANCED_QUOTES);
	// amount of brackets needs to be even
	if (input.open_curly_brackets != input.close_curly_brackets || input.
		open_square_brackets != input.close_square_brackets || input.open_parentheses != input.close_parentheses)
		return (LEXER_UNBALANCED_BRACKETS);
	if ((input.open_curly_brackets + input.close_curly_brackets) % 2 != 0 ||(input.open_square_brackets + input.close_square_brackets) % 2 != 0 || (input.open_parentheses + input.close_parentheses) % 2 != 0)
		return (LEXER_UNBALANCED_BRACKETS);
	// check for this kind of input: echo \"Hello, World!\\\"
	if (str_cchr(s, '\\') % 2 != 0)
		return (LEXER_UNBALANCED_QUOTES);
	// if (check_event_not_found(s))
	// 	return (LEXER_UNBALANCED_QUOTES);
	if (str_cchr(s, '\'') % 2 == 0 && (str_cchr(s, '{') == str_cchr(s, '}')) && (str_cchr(s, '{') + str_cchr(s, '}')) % 2 == 0)
		return (LEXER_SUCCESS);
	return (LEXER_NULL);
}
