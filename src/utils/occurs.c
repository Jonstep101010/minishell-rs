#include "libft.h"

/**
 * @brief looks if little is a substring of big
 *
 * @param big to search in
 * @param little to search for
 * @return char* NULL if not found, invalid input
 */
char	*occurs(const char *big, const char *little)
{
	if (!big || !*big || !little)
		return (NULL);
	return (ft_strnstr(big, little, ft_strlen(big)));
}

/**
 * @brief looks if strings match
 *
 * @param expected
 * @param actual
 * @return char* NULL if not found, invalid input
 */
char	*occurs_exclusively(const char *expected, const char *actual)
{
	if (!expected || !*expected || !actual)
		return (NULL);
	if (ft_strlen(expected) == ft_strlen(actual))
		return (ft_strnstr(expected, actual, ft_strlen(expected)));
	return (NULL);
}
