#include <stdlib.h>

void	*fn_apply_free(void	*a, void *b, void *(*f)(void *, void *))
{
	void	*ptr;

	ptr = f(a, b);
	free(a);
	free(b);
	return (ptr);
}
