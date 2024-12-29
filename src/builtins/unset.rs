use ::libc;

use libft_rs::ft_strchr::ft_strchr;
use libutils_rs::src::array::{arr_free::arr_free, remove_str::rm_str_arr};

use crate::{
	environment::{check_key::check_valid_key, get_index::get_index_env},
	t_shell, t_token,
	tokenizer::build_command::get_cmd_arr_token,
	utils::error::eprint,
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
unsafe extern "C" fn unset_internal(
	mut args: *const *const libc::c_char,
	mut env: *mut *mut libc::c_char,
) -> libc::c_int {
	while !(*args).is_null() {
		if !check_valid_key(*args) || check_illegal_char(*args) as libc::c_int != 0 {
			eprint(
				b"unset: `%s': not a valid identifier\0" as *const u8 as *const libc::c_char,
				*args,
			);
			return 1 as libc::c_int;
		}
		let index = get_index_env(env, *args);
		if index >= 0 as libc::c_int && !(*env.offset(index as isize)).is_null() {
			rm_str_arr(env, *env.offset(index as isize));
		}
		args = args.offset(1);
	}
	0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn builtin_unset(
	mut shell: *mut t_shell,
	mut token: *mut t_token,
) -> libc::c_int {
	let mut args: *mut *const libc::c_char = get_cmd_arr_token(token) as *mut *const libc::c_char;
	if ((*shell).env).is_null() || args.is_null() || (*(*shell).env).is_null() {
		return 0 as libc::c_int;
	}
	if (*args).is_null()
		|| (*args.offset(1 as libc::c_int as isize)).is_null()
		|| **args.offset(1 as libc::c_int as isize) == 0
	{
		arr_free(args as *mut *mut libc::c_char);
		return 0 as libc::c_int;
	}
	let mut status: libc::c_int =
		unset_internal(args.offset(1 as libc::c_int as isize), (*shell).env);
	arr_free(args as *mut *mut libc::c_char);
	status
}
