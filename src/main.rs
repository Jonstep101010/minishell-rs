#![allow(
	dead_code,
	mutable_transmutes,
	non_camel_case_types,
	non_snake_case,
	non_upper_case_globals,
	unused_assignments,
	unused_mut
)]
use ::c2rust_out::*;
extern "C" {
	fn readline(_: *const libc::c_char) -> *mut libc::c_char;
	fn add_history(_: *const libc::c_char);
	fn check_signals(p_termios: *mut termios);
	fn lexer(shell: *mut t_shell, trimmed_line: *const libc::c_char) -> libc::c_int;
	fn execute_commands(shell: *mut t_shell, token: *mut t_token);
	fn init_shell(envp: *const *mut libc::c_char) -> *mut t_shell;
	fn get_input(rl_prompt: *mut libc::c_char) -> *mut libc::c_char;
	fn builtin_exit(shell: *mut t_shell, nullable: *mut t_token) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
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
pub type uint8_t = __uint8_t;
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
pub unsafe extern "C" fn minishell_loop(mut shell: *mut t_shell) {
	let mut trimmed_line: *mut libc::c_char = 0 as *mut libc::c_char;
	let mut readline_line: *mut libc::c_char = 0 as *mut libc::c_char;
	check_signals(&mut (*shell).p_termios);
	loop {
		readline_line = readline(b"minishell> \0" as *const u8 as *const libc::c_char);
		if readline_line.is_null() {
			builtin_exit(shell, 0 as *mut t_token);
		}
		trimmed_line = get_input(readline_line);
		if trimmed_line.is_null() {
			continue;
		}
		add_history(trimmed_line);
		if *trimmed_line == 0 || lexer(shell, trimmed_line) != 0 as libc::c_int {
			continue;
		}
		if !((*shell).env).is_null() && !(*(*shell).env).is_null() && !((*shell).token).is_null() {
			execute_commands(shell, (*shell).token);
		}
	}
}
unsafe fn main_0(
	mut ac: libc::c_int,
	mut av: *mut *mut libc::c_char,
	mut envp: *mut *mut libc::c_char,
) -> libc::c_int {
	let mut shell: *mut t_shell = 0 as *mut t_shell;
	shell = init_shell(envp);
	if shell.is_null() {
		return 1 as libc::c_int;
	}
	minishell_loop(shell);
	return 0 as libc::c_int;
}
pub fn main() {
	let mut args: Vec<*mut libc::c_char> = Vec::new();
	for arg in ::std::env::args() {
		args.push(
			(::std::ffi::CString::new(arg))
				.expect("Failed to convert argument into CString.")
				.into_raw(),
		);
	}
	args.push(::core::ptr::null_mut());
	let mut vars: Vec<*mut libc::c_char> = Vec::new();
	for (var_name, var_value) in ::std::env::vars() {
		let var: String = format!("{}={}", var_name, var_value);
		vars.push(
			(::std::ffi::CString::new(var))
				.expect("Failed to convert environment variable into CString.")
				.into_raw(),
		);
	}
	vars.push(::core::ptr::null_mut());
	unsafe {
		::std::process::exit(main_0(
			(args.len() - 1) as libc::c_int,
			args.as_mut_ptr() as *mut *mut libc::c_char,
			vars.as_mut_ptr() as *mut *mut libc::c_char,
		) as i32)
	}
}
