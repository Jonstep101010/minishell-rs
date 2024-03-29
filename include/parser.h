/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.h                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jschwabe <jschwabe@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/29 18:03:07 by jschwabe          #+#    #+#             */
/*   Updated: 2024/03/29 18:03:10 by jschwabe         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef PARSER_H
# define PARSER_H

# include <stddef.h>
# include <stdbool.h>

typedef struct s_splitter
{
	int			quote;
	size_t		start;
	size_t		len;
	char		**arr;
	char		*to_split;
	const char	*set;
}	t_splitter;

char	**split_outside_quotes(const char *to_split, const char *set);

void	*do_quote_bs(const char *s, int *quote);
bool	interpret_quotes(char **cmd_arr);
#endif