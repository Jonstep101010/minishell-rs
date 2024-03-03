#include "libft.h"

/**
 * @brief looks if strings match
 *
 * @param expected
 * @param actual
 * @return char* NULL if not found, invalid input
 */
char	*equal(const char *expected, const char *actual)
{
	if (!expected || !*expected || !actual)
		return (NULL);
	if (ft_strlen(expected) == ft_strlen(actual))
		return (ft_strnstr(expected, actual, ft_strlen(expected)));
	return (NULL);
}
