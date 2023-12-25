#include "libft.h"
#include "minishell.h"
#include "struct.h"
#include <stdbool.h>

int	str_cchr(const char *s, char c)
{
	int	i;
	int	count;

	i = 0;
	count = 0;
	if (!s)
		return (0);
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
		.doublequotes = str_cchr(s, '"'),
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
		while_d(s, (int (*)(int))&ft_isspace, 1, &i);
		flag = while_d(s, (int (*)(int))&ft_isalnum, 1, &i);
		while_d(s, (int (*)(int))&ft_isspace, 1, &i);
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
	// if (count >= 1)
	if (s[0] == '|')
		return (false);
	while (s[i])
	{
		if (while_not_i(s, (int (*)(int))&ft_isalnum,'|', &i) == 0)
			return (false);
		if (s[i] == '|' && s[i + 1] == '|' && pipes == 2)
			return (two_pipes_valid(s, i));
		while_i(s, (int (*)(int))&ft_isspace, 1, &i);
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

static t_lexer	check_pipes_redirection(const char *s, struct s_lexer *input)
{
	input->lexer = LEXER_PIPES;
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

static t_lexer	check_brackets_quotes(const char *s, struct s_lexer *input)
{
	(void)s;
	input->lexer = LEXER_UNBALANCED_QUOTES;
	if (input->singlequotes % 2 != 0 || input->doublequotes % 2 != 0)
		return (LEXER_UNBALANCED_QUOTES);
	input->lexer = LEXER_UNBALANCED_BRACKETS;
	if (input->open_curly_brackets != input->close_curly_brackets || input->
		open_square_brackets != input->close_square_brackets || input->open_parentheses != input->close_parentheses)
		return (LEXER_UNBALANCED_BRACKETS);
	if ((input->open_curly_brackets + input->close_curly_brackets) % 2 != 0 ||(input->open_square_brackets + input->close_square_brackets) % 2 != 0 || (input->open_parentheses + input->close_parentheses) % 2 != 0)
		return (LEXER_UNBALANCED_BRACKETS);
	return (LEXER_SUCCESS);
}

// t_lexer	input_ignore(const char *s, struct s_lexer *input)
// {

// 	input->lexer = LEXER_NULL;
// 	// if something is in quotes, ignore it
// 	if (input->doublequotes == 0 && input->singlequotes == 0)
// 		return (LEXER_SUCCESS);
// 	range_ignore(s, &(input->ignore), '\'');
// 	range_ignore(s, &(input->ignore), '"');
// 	return (LEXER_SUCCESS);
// }

// t_lexer	check_quotes_ignore(const char *s, struct s_lexer *input)
// {
// 	(void)s;
// 	(void)input;
// 	return (LEXER_SUCCESS);
// }

// "this is my input "ignore" 't' 'this' "is" 'a' "test" 'string'""
// "                 00000000 000 000000 0000 000 000000 000000000"

// handle input like this "this is a cmd "'" hello" (accept in quotes)
// make other checks use the ignore array
// #include "bool_array.c"
t_lexer	lexer(char *s)
{
	if (!s || !*s)
		return (LEXER_NULL);
	// char *s = ft_strtrim(str, " ");
	struct s_lexer	input;
	count_number(s, &input);
	// range_ignore(s, &input.ignore, '\'');
	// if (input.singlequotes > 0 || input.doublequotes > 0)
	// 	input_ignore(s, &input);

	if (check_pipes_redirection(s, &input) != LEXER_SUCCESS)
		return (input.lexer);
	if (check_brackets_quotes(s, &input) != LEXER_SUCCESS)
		return (input.lexer);
	if (input.singlequotes % 2 == 0 && (input.open_curly_brackets == input.close_curly_brackets) && (input.open_curly_brackets + input.close_curly_brackets) % 2 == 0)
		return (LEXER_SUCCESS);
	// if (input.singlequotes % 2 != 0 && input.doublequotes % 2 == 0)
	// 	return (check_quotes_ignore(s, &input));
	return (LEXER_NULL);
}
