#include "libft.h"
#include "minishell.h"

int while_d(const char *s, int (*f)(int), int is_true, int *index)
{
	int i;

	if (!s || !f || !index || *index <= 0 || *index >= (int)ft_strlen(s))
		return (0);
	i = *index;
	while (s[i] && ((f)((int)s[i])) == is_true)
		i--;
	if (i == *index)
		return (0);
	*index = i;
	return (1);
}

int while_i(const char *s, int (*f)(int), int is_true, int *index)
{
	int i;

	if (!s || !f || !index || *index <= 0 || *index >= (int)ft_strlen(s))
		return (0);
	i = *index;
	while (s[i] && ((f)((int)s[i])) == is_true)
		i++;
	if (i == *index)
		return (0);
	*index = i;
	return (1);
}

int	while_not_i(const char *s, int (*f)(int), char c, int *index)
{
	int i;
	int flag;

	if (!s || !f || !index || *index < 0 || *index > (int)ft_strlen(s))
		return (0);
	flag = 0;
	i = *index;
	while (s[i] && s[i] != c)
	{
		if ((f)((int)s[i]) == true)
			flag++;
		i++;
	}
	if (i == *index || flag == 0)
		return (0);
	*index = i;
	return (flag);
}

int	while_is_i(const char *s, char c, int *index)
{
	int i;
	int flag;

	if (!s || !index || *index < 0 || *index > (int)ft_strlen(s))
		return (0);
	flag = 0;
	i = *index;
	while (s[i] && s[i] == c)
	{
		flag++;
		i++;
	}
	if (i == *index || flag == 0)
		return (0);
	*index = i;
	return (flag);
}

