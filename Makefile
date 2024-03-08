NAME		  := minishell

ifeq ($(uname -s),Darwin)
DONE		= printf "\033[0;32m\xE2\x9C\x93\033[0m "
DONE_NL		= printf "\033[0;32m\xE2\x9C\x93\n\033[0m"
else
DONE		= printf "\033[0;32m✓\033[0m "
DONE_NL		= printf "\033[0;32m✓\033[0m\n\n"
endif

# MAKEFLAGS	+= --no-print-directory --silent

CC = clang
# ----------------------------- includes/linking ----------------------------- #
CFLAGS = -Wall -Wextra -Werror -I./include $(shell find ./src -type d | sed 's/^/-I/') $(shell find ./include -name "*.a" -exec dirname {} \; | xargs -I{} find {} -type d | sed 's/^/-I/')

LDFLAGS = ./include/libgnl/libgnl.a ./include/libftprintf/libftprintf.a ./include/libutils/libutils.a ./include/libft/libft.a

# ---------------------------------------------------------------------------- #
#                                 source files                                 #
# ---------------------------------------------------------------------------- #

SRCS = $(addprefix src/builtins/, builtins.c cd.c echo.c env.c exit.c export.c pwd.c unset.c) \
    $(addprefix src/environment/, export_var.c get_env_var.c key.c) \
    $(addprefix src/expander/, expand_var.c expand_variables.c expander.c) \
    $(addprefix src/lexer/, check_pipes.c check_quotes.c checks_basic.c lexer_support.c lexer.c) \
    $(addprefix src/parser/, interpret_quotes.c parser.c split_outside_quotes.c) \
    $(addprefix src/signals/, signals.c) \
    $(addprefix src/tokenizer/, build_command.c build_tokens.c destroy_tokens.c token_utils.c) \
    $(addprefix src/utils/, arr_utils.c bool_array.c error.c free_strjoin.c print_arr_sep.c str_equal.c while_string_wrapper.c) \
    src/init.c src/main.c
OBJS = $(SRCS:.c=.o)

TARGET = minishell

# ---------------------------------------------------------------------------- #
#                                     rules                                    #
# ---------------------------------------------------------------------------- #

LIBS:
	$(MAKE) -C ./include/libft
	$(MAKE) -C ./include/libutils
	$(MAKE) -C ./include/libgnl
	$(MAKE) -C ./include/libftprintf

all: $(TARGET) $(LIBS)
	$(shell mkdir -p build; mv $(OBJS) build)
	$(DONE_NL)

# ----------------------------- additional rules ----------------------------- #
ceedling:
	ceedling release

MEMCHECK_PARAMS = valgrind --leak-check=full --track-origins=yes -s --log-file=valgrind.log 
EXEC_PATH = ./build/release/$(NAME)

memcheck: ceedling
	$(MEMCHECK_PARAMS) $(EXEC_PATH)
memcheck-all: ceedling
	$(MEMCHECK_PARAMS) --show-leak-kinds=all $(EXEC_PATH)

# ---------------------------------------------------------------------------- #
#                                  compilation                                 #
# ---------------------------------------------------------------------------- #
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -lreadline -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf build
	$(MAKE) -C ./include/libft clean
	$(MAKE) -C ./include/libutils clean
	$(MAKE) -C ./include/libgnl clean
	$(MAKE) -C ./include/libftprintf clean
fclean: clean
	rm -f $(TARGET)
	$(MAKE) -C ./include/libft fclean
	$(MAKE) -C ./include/libutils fclean
	$(MAKE) -C ./include/libgnl fclean
	$(MAKE) -C ./include/libftprintf fclean

re: fclean
	git submodule update --init
	$(MAKE)

memtest:
	rm -rf build
	ceedling test:$(TEST)
	valgrind --leak-check=full --track-origins=yes -s --log-file=valgrind.log ./build/test/out/test_$(TEST).out

.PHONY: memtest