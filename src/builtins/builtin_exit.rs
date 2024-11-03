use ::libc;
extern "C" {
	fn get_cmd_arr_token(token: *mut t_token) -> *mut *mut libc::c_char;
	fn destroy_all_tokens(shell: *mut t_shell);
	fn arr_free(arr: *mut *mut libc::c_char);
	fn ft_atol(s: *const libc::c_char) -> libc::c_long;
	fn free(_: *mut libc::c_void);
	fn exit(_: libc::c_int) -> !;
	fn eprint(fmt: *const libc::c_char, _: ...);
	fn eprint_single(fmt: *const libc::c_char, _: ...);
	fn ft_isdigit(c: libc::c_int) -> libc::c_int;
	fn ft_strlen(str: *const libc::c_char) -> size_t;
}
pub type size_t = libc::c_ulong;
use crate::{t_shell, t_token};

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
unsafe extern "C" fn check_sign(mut exit_code: *const libc::c_char) -> bool {
	if *exit_code as libc::c_int == '-' as i32 || *exit_code as libc::c_int == '+' as i32 {
		if *exit_code.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
			return 0 as libc::c_int != 0;
		}
	}
	return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_exit_code(mut command: *mut *const libc::c_char) -> bool {
	let mut exit_code: *const libc::c_char = 0 as *const libc::c_char;
	let mut i: libc::c_int = 0;
	i = -(1 as libc::c_int);
	if command.is_null() || (*command.offset(1 as libc::c_int as isize)).is_null() {
		return 1 as libc::c_int != 0;
	}
	if !(*command.offset(1 as libc::c_int as isize)).is_null()
		&& !(*command.offset(2 as libc::c_int as isize)).is_null()
	{
		eprint(b"exit: too many arguments\0" as *const u8 as *const libc::c_char);
		return 0 as libc::c_int != 0;
	}
	exit_code = *command.offset(1 as libc::c_int as isize);
	if ft_strlen(exit_code) == 1 as libc::c_int as libc::c_ulong
		&& *exit_code as libc::c_int == '0' as i32
	{
		return 1 as libc::c_int != 0;
	}
	loop {
		i += 1;
		if !(*exit_code.offset(i as isize) != 0) {
			break;
		}
		if ft_isdigit(*exit_code.offset(i as isize) as libc::c_int) == 0 && i != 0 as libc::c_int {
			eprint(b"exit: numeric argument required\0" as *const u8 as *const libc::c_char);
			return 0 as libc::c_int != 0;
		}
	}
	if !check_sign(exit_code) {
		eprint(b"exit: numeric argument required\0" as *const u8 as *const libc::c_char);
		return 0 as libc::c_int != 0;
	}
	return 1 as libc::c_int != 0;
}
unsafe extern "C" fn exit_free_internal(mut shell: *mut t_shell, mut exit_code: uint8_t) {
	if !((*shell).env).is_null() {
		arr_free((*shell).env);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(exit_code as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn builtin_exit(
	mut shell: *mut t_shell,
	mut code_nullable: *mut t_token,
) -> libc::c_int {
	let mut exit_code: uint8_t = 0;
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(code_nullable) as *mut *const libc::c_char;
	exit_code = (*shell).exit_status;
	if !code_nullable.is_null() && !command.is_null() {
		if !(*command.offset(1 as libc::c_int as isize)).is_null() {
			if **command.offset(1 as libc::c_int as isize) == 0 {
				eprint_single(b"exit\n\0" as *const u8 as *const libc::c_char);
				eprint(
					b"exit: numeric argument required\0" as *const u8 as *const libc::c_char,
					exit_code as libc::c_int,
				);
				arr_free(command as *mut *mut libc::c_char);
				return 2 as libc::c_int;
			}
			if !check_exit_code(command) {
				arr_free(command as *mut *mut libc::c_char);
				return 1 as libc::c_int;
			}
			exit_code = ft_atol(*command.offset(1 as libc::c_int as isize)) as uint8_t;
		}
		arr_free(command as *mut *mut libc::c_char);
	}
	eprint_single(b"exit\n\0" as *const u8 as *const libc::c_char);
	exit_free_internal(shell, exit_code);
	return 0 as libc::c_int;
}
