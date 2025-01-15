use crate::{
	eprint_msh,
	prelude::*,
	t_shell, t_token,
	tokenizer::{build_command::get_cmd_arr_token, destroy_tokens::destroy_all_tokens},
};

use libft_rs::{ft_isdigit::ft_isdigit, ft_strlen::ft_strlen};
use libutils_rs::src::array::arr_free::arr_free;
use libutils_rs::src::string::ft_atol::ft_atol;

unsafe fn check_sign(exit_code: *const c_char) -> bool {
	let exit_code: *const u8 = exit_code.cast();
	if (*exit_code == b'-' || *exit_code == b'+') && *exit_code.offset(1) == 0 {
		false
	} else {
		true
	}
}
unsafe fn check_exit_code(command: *mut *const c_char) -> bool {
	let mut i = -1;
	if command.is_null() || (*command.offset(1)).is_null() {
		return true;
	}
	if !(*command.offset(1)).is_null() && !(*command.offset(2)).is_null() {
		eprint_msh!("exit: too many arguments");
		return false;
	}
	let exit_code: *const c_char = *command.offset(1);
	if ft_strlen(exit_code) == 1 as c_int as libc::c_ulong && *exit_code as c_int == '0' as i32 {
		return true;
	}
	loop {
		i += 1;
		if *exit_code.offset(i) == 0 {
			break;
		}
		if ft_isdigit(*exit_code.offset(i) as c_int) == 0 && i != 0 {
			eprint_msh!("exit: numeric argument required");
			return false;
		}
	}
	if !check_sign(exit_code) {
		eprint_msh!("exit: numeric argument required");
		return false;
	}
	true
}

#[allow(unused_mut)]
#[unsafe(no_mangle)]
pub unsafe fn builtin_exit(mut shell: *mut t_shell, mut code_nullable: *mut t_token) -> c_int {
	let mut command: *mut *const c_char = get_cmd_arr_token(code_nullable) as *mut *const c_char;
	let mut exit_code: u8 = (*shell).exit_status;
	if !code_nullable.is_null() && !command.is_null() {
		if !(*command.offset(1)).is_null() {
			if **command.offset(1) == 0 {
				// eprintln!("exit");
				eprint_msh!("exit: numeric argument required {}", exit_code,);
				arr_free(command as *mut *mut c_char);
				return 2 as c_int;
			}
			if !check_exit_code(command) {
				arr_free(command as *mut *mut c_char);
				return 1 as c_int;
			}
			exit_code = ft_atol(*command.offset(1)) as u8;
		}
		arr_free(command as *mut *mut c_char);
	}
	eprintln!("exit");
	// exit_free_internal
	{
		destroy_all_tokens(shell);
		// libc::free(shell as *mut libc::c_void);
		std::process::exit(exit_code as i32);
	};
}
