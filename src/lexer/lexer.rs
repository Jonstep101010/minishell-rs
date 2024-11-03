use ::libc;
extern "C" {
	fn tokenize(shell: *mut t_shell, trimmed_line: *const libc::c_char) -> *mut libc::c_void;
	fn destroy_all_tokens(shell: *mut t_shell);
	fn update_exit_status(shell: *mut t_shell, status: libc::c_int);
	fn free(_: *mut libc::c_void);
	fn lexer_checks_basic(s: *const libc::c_char) -> *mut t_lexer;
	fn get_input(rl_prompt: *mut libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_lexer {
	pub singlequotes: libc::c_int,
	pub doublequotes: libc::c_int,
	pub open_curly_brackets: libc::c_int,
	pub close_curly_brackets: libc::c_int,
	pub open_square_brackets: libc::c_int,
	pub close_square_brackets: libc::c_int,
	pub open_parentheses: libc::c_int,
	pub close_parentheses: libc::c_int,
	pub redir_greater: libc::c_int,
	pub redir_smaller: libc::c_int,
	pub pipes: libc::c_int,
	pub ignore: *mut bool,
	pub len: size_t,
	pub lexer: libc::c_int,
	pub result: bool,
}
pub type t_lexer = s_lexer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_token {
	pub cmd_args: *mut t_arg,
	pub has_redir: bool,
	pub split_pipes: *mut libc::c_char,
	pub tmp_arr: *mut *mut libc::c_char,
	pub bin: *mut libc::c_char,
	pub cmd_func: Option<unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int>,
}
pub type t_token = s_token;
pub type t_shell = s_shell;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_shell {
	pub exit_status: uint8_t,
	pub env: *mut *mut libc::c_char,
	pub token: *mut t_token,
	pub token_len: size_t,
	pub p_termios: termios,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
	pub c_iflag: tcflag_t,
	pub c_oflag: tcflag_t,
	pub c_cflag: tcflag_t,
	pub c_lflag: tcflag_t,
	pub c_line: cc_t,
	pub c_cc: [cc_t; 32],
	pub c_ispeed: speed_t,
	pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type t_arg = s_arg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_arg {
	pub elem: *mut libc::c_char,
	pub type_0: e_arg,
	pub redir: e_redir,
}
pub type e_redir = libc::c_uint;
pub const HEREDOC: e_redir = 4;
pub const APPEND: e_redir = 3;
pub const OUTPUT_REDIR: e_redir = 2;
pub const INPUT_REDIR: e_redir = 1;
pub const NO_REDIR: e_redir = 0;
pub type e_arg = libc::c_uint;
pub const REDIR_REMOVED: e_arg = 2;
pub const REDIR: e_arg = 1;
pub const STRING: e_arg = 0;
#[no_mangle]
pub unsafe extern "C" fn lexer(
	mut shell: *mut t_shell,
	mut trimmed_line: *const libc::c_char,
) -> libc::c_int {
	let mut lex: *mut t_lexer = 0 as *mut t_lexer;
	if *trimmed_line == 0 {
		get_input(0 as *mut libc::c_char);
		return 0 as libc::c_int;
	}
	lex = lexer_checks_basic(trimmed_line);
	if !(*lex).result {
		update_exit_status(shell, (*lex).lexer);
		get_input(0 as *mut libc::c_char);
		free(lex as *mut libc::c_void);
		return 1 as libc::c_int;
	}
	free(lex as *mut libc::c_void);
	(*shell).token = tokenize(shell, trimmed_line) as *mut t_token;
	get_input(0 as *mut libc::c_char);
	if ((*shell).token).is_null() {
		return -(1 as libc::c_int);
	}
	if ((*(*shell).token).cmd_args).is_null() {
		destroy_all_tokens(shell);
		return -(1 as libc::c_int);
	}
	return 0 as libc::c_int;
}
