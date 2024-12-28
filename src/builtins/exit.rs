use crate::{
	eprint_msh, t_shell, t_token,
	tokenizer::{build_command::get_cmd_arr_token, destroy_tokens::destroy_all_tokens},
};
use ::libc;
use libc::{exit, free};

use libft_rs::{ft_isdigit::ft_isdigit, ft_strlen::ft_strlen};
use libutils_rs::src::array::arr_free::arr_free;
use libutils_rs::src::string::ft_atol::ft_atol;

unsafe extern "C" fn check_sign(mut exit_code: *const libc::c_char) -> bool {
	if (*exit_code as libc::c_int == '-' as i32 || *exit_code as libc::c_int == '+' as i32)
		&& *exit_code.offset(1) as libc::c_int == 0 as libc::c_int
	{
		return 0 as libc::c_int != 0;
	}
	1 as libc::c_int != 0
}
unsafe extern "C" fn check_exit_code(mut command: *mut *const libc::c_char) -> bool {
	let mut i: libc::c_int = -1;
	if command.is_null() || (*command.offset(1)).is_null() {
		return 1 as libc::c_int != 0;
	}
	if !(*command.offset(1)).is_null() && !(*command.offset(2)).is_null() {
		eprint_msh!("exit: too many arguments");
		return 0 as libc::c_int != 0;
	}
	let mut exit_code: *const libc::c_char = *command.offset(1);
	if ft_strlen(exit_code) == 1 as libc::c_int as libc::c_ulong
		&& *exit_code as libc::c_int == '0' as i32
	{
		return 1 as libc::c_int != 0;
	}
	loop {
		i += 1;
		if *exit_code.offset(i as isize) == 0 {
			break;
		}
		if ft_isdigit(*exit_code.offset(i as isize) as libc::c_int) == 0 && i != 0 as libc::c_int {
			eprint_msh!("exit: numeric argument required");
			return 0 as libc::c_int != 0;
		}
	}
	if !check_sign(exit_code) {
		eprint_msh!("exit: numeric argument required");
		return 0 as libc::c_int != 0;
	}
	1 as libc::c_int != 0
}
unsafe extern "C" fn exit_free_internal(mut shell: *mut t_shell, mut exit_code: u8) {
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
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(code_nullable) as *mut *const libc::c_char;
	let mut exit_code: u8 = (*shell).exit_status;
	if !code_nullable.is_null() && !command.is_null() {
		if !(*command.offset(1)).is_null() {
			if **command.offset(1) == 0 {
				// eprintln!("exit");
				eprint_msh!("exit: numeric argument required {}", exit_code,);
				arr_free(command as *mut *mut libc::c_char);
				return 2 as libc::c_int;
			}
			if !check_exit_code(command) {
				arr_free(command as *mut *mut libc::c_char);
				return 1 as libc::c_int;
			}
			exit_code = ft_atol(*command.offset(1)) as u8;
		}
		arr_free(command as *mut *mut libc::c_char);
	}
	eprintln!("exit");
	exit_free_internal(shell, exit_code);
	0 as libc::c_int
}
