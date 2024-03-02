#include <stddef.h>
#include "libft.h"

// @todo write a function strtrim_outside_quotes to only trim outside quotes
// char	*ft_strtrim(char const *s, char const *set)
// {
// 	char	*trim;
// 	size_t	slen;

// 	if (!s)
// 		return (NULL);
// 	if (!set)
// 		return ((char *)s);
// 	while ((*s) && ft_strchr(set, *s))
// 		s++;
// 	slen = ft_strlen(s);
// 	while ((slen > 0) && ft_strrchr(set, s[slen - 1]))
// 		slen--;
// 	trim = ft_substr(s, 0, slen);
// 	if (!trim)
// 		return (NULL);
// 	return (trim);
// }
#ifndef TEST
int is_in_set_strtrim(char c, const char *set)
{
	while (*set)
	{
		if (c == *set)
			return (1);
		set++;
	}
	return (0);
}
#endif
char	*strtrim_outside_quotes(char const *s, char const *set)
{
	char	*trim;
	size_t	slen;

	if (!s)
		return (NULL);
	if (!set)
		return ((char *)s);
	while ((*s) && is_in_set_strtrim(*s, set) && *s != '\'' && *s != '"')
		s++;
	if (!*s)
		return (ft_strdup(""));
	slen = ft_strlen(s) - 1;
	while ((slen > 0) && is_in_set_strtrim(s[slen - 1], set) && s[slen - 1] != '\'' && s[slen - 1] != '"')
		slen--;
	trim = ft_substr(s, 0, slen);
	if (!trim)
		return (NULL);
	return (trim);
}

char	*strtrim_front_outside_quotes(char const *s, char const *set)
{
	char	*trim;
	size_t	slen;

	if (!s)
		return (NULL);
	if (!set)
		return ((char *)s);
	while ((*s) && is_in_set_strtrim(*s, set) && *s != '\'' && *s != '"')
		s++;
	if (!*s)
		return (ft_strdup(""));
	slen = ft_strlen(s);
	trim = ft_substr(s, 0, slen);
	if (!trim)
		return (NULL);
	return (trim);
}

char	*strtrim_back_outside_quotes(char const *s, char const *set)
{
	char	*trim;
	size_t	slen;

	if (!s)
		return (NULL);
	if (!set)
		return ((char *)s);
	slen = ft_strlen(s);
	while ((slen > 0) && is_in_set_strtrim(s[slen - 1], set) && s[slen - 1] != '\'' && s[slen - 1] != '"')
		slen--;
	trim = ft_substr(s, 0, slen);
	if (!trim)
		return (NULL);
	return (trim);
}
