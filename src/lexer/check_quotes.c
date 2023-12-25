#include "lexer.h"
#include "libft.h"

void debug_print_table(const char *s, const bool *arr) {
	if (!s || !arr)
		return ;
	for (int i = 0; i <= (int)strlen(s); i++){
		fprintf(stderr, "i: [%d]: %d, %c\n", i, arr[i], s[i]);
	}
	for (int i = 0; i <= (int)strlen(s); i++){
		if (arr[i] == true)
			fprintf(stderr, "%d", arr[i]);
		else
			fprintf(stderr, "%d", arr[i]);
	}
	fprintf(stderr,"\n");
}

void debug_print_line(const char *s, const bool *arr) {
	if (!s || !arr)
		return ;
	for (int i = 0; i <= (int)strlen(s); i++){
		fprintf(stderr, "%c", s[i]);
	}
	fprintf(stderr,"\n");
	for (int i = 0; i <= (int)strlen(s); i++){
		if (arr[i] == true)
			fprintf(stderr, "%d", arr[i]);
		else
			fprintf(stderr, "%d", arr[i]);
	}
	fprintf(stderr,"\n");
}

t_lexer	check_quotes_ignore(const char *s, struct s_lexer *input)
{
	input->ignore = NULL;
	bool_arr_zeroing(s, &(input->ignore), ft_strlen(s));
	range_ignore(s, &(input->ignore),'\'');
	range_ignore(s, &(input->ignore),'"');
	debug_print_line(s, input->ignore);
	return (LEXER_SUCCESS);
}
