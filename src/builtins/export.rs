use crate::prelude::*;
use crate::{
	environment::check_key::check_valid_key, environment::Env, eprint_msh, size_t, t_shell,
	t_token, tokenizer::build_command::get_cmd_arr_token,
};
use ::libc;

use libutils_rs::src::{array::arr_free::arr_free, string::str_cchr::str_cchr};

fn declare_x(env: &Env) {
	for (key, value) in env.iter() {
		println!("declare -x {}={}", key, value);
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn builtin_export(
	mut shell: *mut t_shell,
	mut token: *mut t_token,
) -> libc::c_int {
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(token) as *mut *const libc::c_char;
	let mut i: size_t = 1;
	if command.is_null() || (*command.offset(i as isize)).is_null() {
		arr_free(command as *mut *mut libc::c_char);
		declare_x(&(*shell).env);
		return 0;
	}
	while !(*command.offset(i as isize)).is_null() {
		if !check_valid_key(*command.offset(i as isize)) {
			// @audit does this work?
			let faulty_identifier = i8const_str(command, i);

			eprint_msh!("export: `{}': not a valid identifier", faulty_identifier);
			arr_free(command as *mut *mut libc::c_char);
			return 1 as libc::c_int;
		}
		if str_cchr(*command.offset(i as isize), '=' as i32 as libc::c_char) >= 1 as libc::c_int {
			let kv = i8const_str(command, i);
			let (key, value) = kv.split_once('=').unwrap();
			(*shell).env.export(key, value.to_string())
		}
		i = i.wrapping_add(1);
	}
	arr_free(command as *mut *mut libc::c_char);
	if i > 1 as libc::c_int as libc::c_ulong {
		return 0 as libc::c_int;
	}
	1 as libc::c_int
}
