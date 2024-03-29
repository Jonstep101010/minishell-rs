/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   check_pipes.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:55:26 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 19:43:23 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "lexer.h"
#include "minishell.h"
#include "libft.h"
#include "libutils.h"
#include "utils.h"

static int	inner_while_quotes(
	const char *s, t_lexer *input, struct s_check_pipes *check)
{
	check->flag_word = 0;
	check->flag_redir = 0;
	while (s[check->i] && s[check->i] != '|' && !input->ignore[check->i])
	{
		if (ft_strchr("><", s[check->i])
			&& (!check->flag_redir || (s[check->i - 1]
					&& s[check->i - 1] == s[check->i])))
			check->flag_redir = 1;
		else if (ft_strchr("><", s[check->i]))
			return (eprint("%s %s", ERR_TOKEN, "`newline'"), 2);
		else if (ft_isalnum(s[check->i]))
		{
			check->flag_redir = 0;
			check->flag_word = 1;
		}
		check->i++;
	}
	return (0);
}

static int	inner_if_quotes(
	const char *s, t_lexer *input, struct s_check_pipes *check)
{
	if (!input->ignore[check->i] && s[check->i] == '|'
		&& !check->ignore)
	{
		if (!check->flag_word)
			return (eprint("%s %s", ERR_TOKEN, "`|'"), 2);
		if ((!s[check->i] || s[check->i] == '|')
			&& (check->flag_redir || !check->flag_word))
			return (eprint("%s %s", ERR_TOKEN, "`|'"), 2);
	}
	if (s[check->i] == '|')
		check->ignore = false;
	return (0);
}

/**
 * @brief Check for pipes, redirections and quotes
 * @details ignored means that anything inside can be a valid
 * command/word for redirection, after redir makes it valid
 * same for before or after pipe (not enclosed in quotes)
 * @return int 0 on success, 2 on error
 */
static int	check_pipes_redirection_quotes(
	const char *s, t_lexer *input, struct s_check_pipes *check)
{
	while (check->i < input->len && input->ignore)
	{
		if (!input->ignore[check->i])
		{
			if (inner_while_quotes(s, input, check) != 0
				|| inner_if_quotes(s, input, check) != 0)
				return (2);
		}
		if (input->ignore[check->i])
		{
			check->ignore = true;
			while (s[check->i] && input->ignore[check->i])
				check->i++;
		}
		else
			check->i++;
	}
	if (check->flag_redir && !check->ignore)
		return (eprint("%s %s", ERR_TOKEN, "`newline'"), 2);
	return (LEXER_SUCCESS);
}

static int	inner_while_noquotes(const char *s, struct s_check_pipes *check)
{
	while (s[check->i] && s[check->i] != '|')
	{
		if (ft_strchr("><", s[check->i])
			&& (!check->flag_redir || (s[check->i - 1]
					&& s[check->i - 1] == s[check->i]
					&& (!s[check->i - 2] || ft_isspace(s[check->i - 2])))))
			check->flag_redir = 1;
		else if (ft_strchr("><", s[check->i]))
			return (eprint("%s %s", ERR_TOKEN, "`newline'"), 2);
		else if (ft_isalnum(s[check->i]))
		{
			check->flag_redir = 0;
			check->flag_word = 1;
		}
		check->i++;
	}
	return (0);
}

int	check_pipes_redirection(const char *s, t_lexer *input)
{
	struct s_check_pipes	check;

	check = (struct s_check_pipes){0, 0, 0, false};
	if (*s == '|' || s[input->len - 1] == '|')
		return (eprint("%s %s", ERR_TOKEN, "`|'"), 2);
	if (ft_strchr("<>", s[input->len - 1]))
		return (eprint("%s %s", ERR_TOKEN, "`newline'"), 2);
	if (input->ignore)
		return (check_pipes_redirection_quotes(s, input, &check));
	while (check.i < input->len)
	{
		check = (struct s_check_pipes){.i = check.i};
		if (inner_while_noquotes(s, &check) != 0)
			return (2);
		if (!check.flag_word)
			return (eprint("%s %s", ERR_TOKEN, "`|'"), 2);
		if ((!s[check.i] || s[check.i] == '|')
			&& (check.flag_redir || !check.flag_word))
			return (eprint("%s %s", ERR_TOKEN, "`|'"), 2);
		while (s[check.i] && s[check.i] == '|')
			check.i++;
	}
	if (check.flag_redir)
		return (eprint("%s %s", ERR_TOKEN, "`newline'"), 2);
	return (LEXER_SUCCESS);
}
