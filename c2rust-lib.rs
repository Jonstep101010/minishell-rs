#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]

extern crate libc;
pub mod include {
	pub mod libft_rs {
		pub mod ft_atoi;
		pub mod ft_bzero;
		pub mod ft_calloc;
		pub mod ft_isalnum;
		pub mod ft_isalpha;
		pub mod ft_isascii;
		pub mod ft_isdigit;
		pub mod ft_isprint;
		pub mod ft_itoa;
		pub mod ft_memchr;
		pub mod ft_memcmp;
		pub mod ft_memcpy;
		pub mod ft_memmove;
		pub mod ft_memset;
		pub mod ft_putchar_fd;
		pub mod ft_putendl_fd;
		pub mod ft_putnbr_fd;
		pub mod ft_putstr_fd;
		pub mod ft_split;
		pub mod ft_strchr;
		pub mod ft_strdup;
		pub mod ft_striteri;
		pub mod ft_strjoin;
		pub mod ft_strlcat;
		pub mod ft_strlcpy;
		pub mod ft_strlen;
		pub mod ft_strmapi;
		pub mod ft_strncmp;
		pub mod ft_strnstr;
		pub mod ft_strrchr;
		pub mod ft_strtrim;
		pub mod ft_substr;
		pub mod ft_tolower;
		pub mod ft_toupper;
	} // mod libft_rs
	pub mod libftprintf_rs {
		pub mod ft_ltoa;
		pub mod ft_printf;
		pub mod ft_vdprintf;
		pub mod put_utils;
	} // mod libftprintf_rs
	pub mod libgnl_rs {
		pub mod get_next_line;
		pub mod get_next_line_utils;
	} // mod libgnl_rs
	pub mod libutils_rs {
		pub mod src {
			pub mod array {
				pub mod append_str;
				pub mod arr_dup;
				pub mod arr_free;
				pub mod arr_len;
				pub mod remove_str;
			} // mod array
			pub mod r#char {
				pub mod is_something;
			} // mod r#char
			pub mod string {
				pub mod append_char;
				pub mod free_strjoin;
				pub mod free_strjoin_utils;
				pub mod ft_atol;
				pub mod idx_strchr;
				pub mod join_strings;
				pub mod secure_strlen;
				pub mod str_cchr;
				pub mod str_equal;
			} // mod string
			pub mod utils {
				pub mod free_mem;
				pub mod memsize;
			} // mod utils
		} // mod src
	} // mod libutils_rs
} // mod include
pub mod src {
	pub mod builtins {
		pub mod builtin_cd;
		pub mod builtin_echo;
		pub mod builtin_env;
		pub mod builtin_exit;
		pub mod builtin_export;
		pub mod builtin_pwd;
		pub mod builtin_unset;
	} // mod builtins
	pub mod environment {
		pub mod check_key;
		pub mod expander;
		pub mod export_env;
		pub mod get_env;
		pub mod get_index;
	} // mod environment
	pub mod execution {
		pub mod bin_path;
		pub mod exec_bin;
		pub mod execute_commands;
		pub mod execute_pipes;
		pub mod heredoc;
		pub mod redirections;
	} // mod execution
	pub mod lexer {
		pub mod check_pipes;
		pub mod checks_basic;
		pub mod lexer;
		pub mod lexer_support;
	} // mod lexer
	pub mod parser {
		pub mod interpret_quotes;
		pub mod split_outside_quotes;
	} // mod parser
	pub mod signals {
		pub mod signals;
		pub mod signals_childs;
	} // mod signals
	pub mod tokenizer {
		pub mod build_command;
		pub mod build_tokens;
		pub mod destroy_tokens;
		pub mod redirection_utils;
		pub mod token_utils;
	} // mod tokenizer
	pub mod utils {
		pub mod arr_utils;
		pub mod bool_array;
		pub mod error;
		pub mod exit_free;
		pub mod get_input;
		pub mod init_shell;
		pub mod str_equal;
	} // mod utils
} // mod src
