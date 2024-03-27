#include "lexer.h"
#include "minishell.h"
#include "libft.h"
#include "libutils.h"
#include "utils.h"

int	check_pipes_redirection_quotes(const char *s, t_lexer *input)
{
	if (!input->ignore)
		return (LEXER_NULL);
	int	flag_redir = 0;
	int	flag_word = 0;
	if (*s == '|')
		return (eprint("found leading pipe"), ERR_LEADING);
	fprintf(stderr, "s: %s\n", s);
	if (ft_strchr("<>|", s[input->len - 1]))
		return (eprint("found trailing redir/pipe"), ERR_TRAILING);
	size_t	i = 0;
	while (i < input->len && input->ignore)
	{
		if (input->ignore && !input->ignore[i])
		{
			flag_word = 0;
			flag_redir = 0;
			while (s[i] && s[i] != '|' && !input->ignore[i])
			{
				if (ft_strchr("><", s[i]) && (!flag_redir || (s[i - 1] && s[i - 1] == s[i] && (!s[i - 2] || ft_isspace(s[i - 2])))))
					flag_redir = 1;
				else if (ft_strchr("><", s[i]))
					return (eprint("syntax error near unexpected token `newline'"), 2);
				else if (ft_isalnum(s[i]))
				{
					flag_redir = 0;
					flag_word = 1;
				}
				i++;
			}
			if (input->ignore[i])
				break ;
			else if (!flag_word)
				return (eprint("syntax error near unexpected token `|'"), 2);
			else if ((!s[i] || s[i] == '|') && (flag_redir || !flag_word))
				return (eprint("syntax error near unexpected token `|'"), 2);
			while (s[i] && s[i] == '|')
				i++;
		}
		else
			i++;
	}
	if (flag_redir)
		return (eprint("syntax error near unexpected token `newline'"), 2);
	return (LEXER_SUCCESS);
}

int	check_pipes_redirection(const char *s, t_lexer *input)
{
	int	flag_redir = 0;
	int	flag_word = 0;
	if (input->ignore)
		return (check_pipes_redirection_quotes(s, input));
	if (*s == '|')
		return (eprint("found leading pipe"), ERR_LEADING);
	fprintf(stderr, "s: %s\n", s);
	if (ft_strchr("<>|", s[input->len - 1]))
		return (eprint("found trailing redir/pipe"), ERR_TRAILING);
	size_t	i = 0;
	while (i < input->len)
	{
		if (s[i])
		{
			flag_word = 0;
			flag_redir = 0;
			while (s[i] && s[i] != '|')
			{
				if (ft_strchr("><", s[i]) && (!flag_redir || (s[i - 1] && s[i - 1] == s[i] && (!s[i - 2] || ft_isspace(s[i - 2])))))
					flag_redir = 1;
				else if (ft_strchr("><", s[i]))
					return (eprint("syntax error near unexpected token `newline'"), 2);
				else if (ft_isalnum(s[i]))
				{
					flag_redir = 0;
					flag_word = 1;
				}
				i++;
			}
			if (!flag_word)
				return (eprint("syntax error near unexpected token `|'"), 2);
			if ((!s[i] || s[i] == '|') && (flag_redir || !flag_word))
				return (eprint("syntax error near unexpected token `|'"), 2);
			while (s[i] && s[i] == '|')
				i++;
		}
		else
			i++;
	}
	if (flag_redir)
		return (eprint("syntax error near unexpected token `newline'"), 2);
	return (LEXER_SUCCESS);
}
