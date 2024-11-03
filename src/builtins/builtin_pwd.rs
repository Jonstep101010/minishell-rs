use ::libc;
extern "C" {
	fn get_env(env: *const *mut libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
	fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
	fn free(_: *mut libc::c_void);
	fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn builtin_pwd(
	mut shell: *mut t_shell,
	mut token_nullable: *mut t_token,
) -> libc::c_int {
	let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
	let mut env_pwd: *mut libc::c_char = 0 as *mut libc::c_char;
	tmp = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
	env_pwd = get_env((*shell).env, b"PWD\0" as *const u8 as *const libc::c_char);
	if env_pwd.is_null() {
		printf(b"%s\n\0" as *const u8 as *const libc::c_char, tmp);
	} else {
		printf(b"%s\n\0" as *const u8 as *const libc::c_char, env_pwd);
	}
	if !tmp.is_null() {
		free(tmp as *mut libc::c_void);
	}
	if !env_pwd.is_null() {
		free(env_pwd as *mut libc::c_void);
	}
	return 0 as libc::c_int;
}
