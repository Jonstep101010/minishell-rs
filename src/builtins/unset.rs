use ::libc;

use libft_rs::ft_strchr::ft_strchr;
use libutils_rs::src::array::arr_free::arr_free;

use crate::{
	environment::{Env, check_key::check_valid_key},
	eprint_msh, i8const_str, t_shell, t_token,
	tokenizer::build_command::get_cmd_arr_token,
};

unsafe extern "C" fn check_illegal_char(mut str: *const libc::c_char) -> bool {
	while *str != 0 {
		if !(ft_strchr(
			b"?=;$.:><[]()/+-{}*#@!^\0" as *const u8 as *const libc::c_char,
			*str as libc::c_int,
		))
		.is_null()
		{
			return 1 as libc::c_int != 0;
		}
		str = str.offset(1);
	}
	0 as libc::c_int != 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn builtin_unset(
	mut shell: *mut t_shell,
	mut token: *mut t_token,
) -> libc::c_int {
	let mut args: *mut *const libc::c_char = get_cmd_arr_token(token) as *mut *const libc::c_char;
	if args.is_null() {
		return 0 as libc::c_int;
	}
	if (*args).is_null()
		|| (*args.offset(1 as libc::c_int as isize)).is_null()
		|| **args.offset(1 as libc::c_int as isize) == 0
	{
		arr_free(args as *mut *mut libc::c_char);
		return 0 as libc::c_int;
	}
	let mut status: libc::c_int = {
		let mut args: *const *const libc::c_char = args.offset(1 as libc::c_int as isize);
		let mut env: &mut Env = &mut (*shell).env;
		while !(*args).is_null() {
			if !check_valid_key(*args) || check_illegal_char(*args) as libc::c_int != 0 {
				let faulty = i8const_str(args, 0);
				eprint_msh!("unset: `{faulty}': not a valid identifier");
				return 1 as libc::c_int;
			}
			let rm_key = i8const_str(args, 0);
			if let Some(key) = env.remove(rm_key) {
				println!("unset: {}", key);
			} else {
				eprint_msh!("unset: `{}': not a valid identifier", rm_key);
				return 1 as libc::c_int;
			}
			// remove an element from the environment
			args = args.offset(1);
		}
		0 as libc::c_int
	};
	arr_free(args as *mut *mut libc::c_char);
	status
}
