use crate::prelude::*;

use ::libc;

use libutils_rs::src::{array::arr_free::arr_free, string::str_cchr::str_cchr};

fn declare_x(env: &Env) {
	for (key, value) in env.iter() {
		println!("declare -x {}={}", key, value);
	}
}

#[unsafe(no_mangle)]
pub unsafe fn builtin_export(shell_env: &mut Env, command: *mut *const c_char) -> i32 {
	let mut i = 1;
	if (*command.add(i)).is_null() {
		arr_free(command as *mut *mut libc::c_char);
		declare_x(shell_env);
		return 0;
	}
	while !(*command.add(i)).is_null() {
		if !crate::environment::check_key::check_valid_key(*command.add(i)) {
			// @audit does this work?
			let faulty_identifier = i8const_str(command, i as u64);

			eprint_msh!("export: `{}': not a valid identifier", faulty_identifier);
			arr_free(command as *mut *mut libc::c_char);
			return 1;
		}
		if str_cchr(*command.add(i), '=' as i32 as libc::c_char) >= 1 {
			let kv = i8const_str(command, i as u64);
			let (key, value) = kv.split_once('=').unwrap();
			shell_env.export(key, value.to_string())
		}
		i += 1;
	}
	arr_free(command as *mut *mut libc::c_char);
	if i > 1 {
		return 0;
	}
	1
}
