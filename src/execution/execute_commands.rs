use ::libc;
extern "C" {
	fn destroy_all_tokens(shell: *mut t_shell);
	fn update_exit_status(shell: *mut t_shell, status: libc::c_int);
	fn builtin_cd(shell: *mut t_shell, token: *mut t_token) -> libc::c_int;
	fn builtin_export(shell: *mut t_shell, token: *mut t_token) -> libc::c_int;
	fn builtin_unset(shell: *mut t_shell, token: *mut t_token) -> libc::c_int;
	fn builtin_exit(shell: *mut t_shell, nullable: *mut t_token) -> libc::c_int;
	fn strerror(_: libc::c_int) -> *mut libc::c_char;
	fn eprint(fmt: *const libc::c_char, _: ...);
	fn __errno_location() -> *mut libc::c_int;
	fn do_redirections(cmd_args: *mut t_arg, error_elem: *mut *mut libc::c_char) -> libc::c_int;
	fn execute_pipes(shell: *mut t_shell, token_count: libc::c_int);
	fn memsize(arr: *mut libc::c_void, size: size_t) -> size_t;
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
unsafe extern "C" fn forkable_builtin(mut token: *mut t_token) -> bool {
	return (*token).cmd_func
		!= Some(builtin_exit as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int)
		&& (*token).cmd_func
			!= Some(
				builtin_export as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
			) && (*token).cmd_func
		!= Some(builtin_unset as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int)
		&& (*token).cmd_func
			!= Some(builtin_cd as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn execute_commands(mut shell: *mut t_shell, mut token: *mut t_token) {
	let mut token_count: libc::c_int = 0;
	let mut redir_status: libc::c_int = 0;
	let mut error_elem: *mut libc::c_char = 0 as *mut libc::c_char;
	if token.is_null() {
		return update_exit_status(shell, -(1 as libc::c_int));
	}
	token_count = memsize(
		(*shell).token as *mut libc::c_void,
		::core::mem::size_of::<t_token>() as libc::c_ulong,
	) as libc::c_int;
	if token_count == 1 as libc::c_int && !forkable_builtin(token) {
		redir_status = do_redirections((*token).cmd_args, &mut error_elem);
		if redir_status != 0 as libc::c_int {
			eprint(
				b"%s: %s\0" as *const u8 as *const libc::c_char,
				error_elem,
				strerror(*__errno_location()),
			);
			return update_exit_status(shell, redir_status);
		}
		update_exit_status(
			shell,
			((*token).cmd_func).expect("non-null function pointer")(shell, token),
		);
	} else {
		execute_pipes(shell, token_count);
	}
	destroy_all_tokens(shell);
}
