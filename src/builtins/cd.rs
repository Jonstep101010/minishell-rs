use ::libc;

use crate::{environment::Env, t_shell, t_token, tokenizer::build_command::get_cmd_arr_token};
use libutils_rs::src::array::arr_free::arr_free;
use std::{ffi::CStr, path::Path};

fn changedir(mut path_string: &str, mut env: &mut Env) -> bool {
	let oldpwd = std::env::current_dir().unwrap();
	let rust_path = Path::new(path_string);
	match std::env::set_current_dir(rust_path) {
		Ok(_) => {
			let pwd = std::env::current_dir();
			match pwd {
				Ok(p) => {
					env.export("PWD", p.to_str().unwrap().to_string());
					env.export("PWD", oldpwd.to_str().unwrap().to_string());
					true
				}
				Err(e) => {
					eprintln!("cd: {}: {}", path_string, e);
					false
				}
			}
		}
		Err(e) => {
			eprintln!("cd: {}: {}", path_string, e);
			false
		}
	}
}

fn cd_internal(mut opt_cmd_args: Option<&str>, mut env: &mut Env) -> bool {
	if opt_cmd_args.is_none() {
		let env_path = env.get("HOME");
		if env_path.is_none() {
			eprintln!("cd: HOME not set");
			false
		} else {
			let env_path = env.get("HOME").unwrap().clone();
			changedir(&env_path, &mut env);
			true
		}
	} else {
		let env_path = env.get("HOME");
		if env_path.is_some() && opt_cmd_args.unwrap().as_bytes() == b"~" {
			if !changedir(&env_path.unwrap().clone(), &mut env) {
				false
			} else {
				true
			}
		} else if opt_cmd_args.unwrap().as_bytes() == b"-" && env.get("OLDPWD").is_some() {
			let oldpwd = env.get("OLDPWD").unwrap().clone();
			if !changedir(&oldpwd, &mut env) {
				false
			} else {
				true
			}
		} else if !changedir(&opt_cmd_args.unwrap().to_string(), &mut env) {
			false
		} else {
			true
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn builtin_cd(
	mut shell: *mut t_shell,
	mut token: *mut t_token,
) -> libc::c_int {
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(token) as *mut *const libc::c_char;
	let cmd_args = *command.offset(1);
	// option of cmd_args -> none if null
	let opt_cmd_args = if cmd_args.is_null() {
		None
	} else {
		Some(CStr::from_ptr(cmd_args).to_str().unwrap())
	};
	let mut shell_env: &mut Env = &mut (*shell).env;
	let status = cd_internal(opt_cmd_args, shell_env);
	arr_free(command as *mut *mut libc::c_char);
	!status as libc::c_int
}
