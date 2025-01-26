use ::libc;

use libft_rs::ft_strchr::ft_strchr;
use libutils_rs::src::array::arr_free::arr_free;

use crate::{
	environment::{Env, check_key::check_valid_key},
	eprint_msh, i8const_str,
	prelude::*,
};

unsafe fn check_illegal_char(mut str: *const libc::c_char) -> bool {
	while *str != 0 {
		if !(ft_strchr(c"?=;$.:><[]()/+-{}*#@!^".as_ptr(), *str as i32)).is_null() {
			return true;
		}
		str = str.add(1);
	}
	false
}

#[unsafe(no_mangle)]
pub unsafe fn builtin_unset(shell_env: &mut Env, cmd_opt: Option<*mut *const c_char>) -> i32 {
	if let Some(args) = cmd_opt {
		if (*args).is_null() || (*args.add(1)).is_null() || **args.add(1) == 0 {
			arr_free(args as *mut *mut libc::c_char);
			return 0;
		}
		let status: i32 = {
			let mut args: *const *const libc::c_char = args.add(1);
			while !(*args).is_null() {
				if !check_valid_key(*args) || check_illegal_char(*args) as i32 != 0 {
					let faulty = i8const_str(args, 0);
					eprint_msh!("unset: `{faulty}': not a valid identifier");
					return 1;
				}
				let rm_key = i8const_str(args, 0);
				if let Some(key) = shell_env.remove(rm_key) {
					println!("unset: {}", key);
				} else {
					eprint_msh!("unset: `{}': not a valid identifier", rm_key);
					return 1;
				}
				// remove an element from the environment
				args = args.add(1);
			}
			0
		};
		arr_free(args as *mut *mut libc::c_char);
		return status;
	}
	1
}
