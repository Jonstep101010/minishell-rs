use ::libc;

use crate::prelude::*;
use libutils_rs::src::array::arr_free::arr_free;
use std::{ffi::CStr, path::Path};

fn changedir(path_string: &str, env: &mut Env) -> bool {
	let oldpwd = std::env::current_dir().unwrap();
	let rust_path = Path::new(path_string);
	match std::env::set_current_dir(rust_path) {
		Ok(_) => {
			let pwd = std::env::current_dir();
			match pwd {
				Ok(p) => {
					env.export("PWD", p.to_str().unwrap().to_string());
					env.export("OLDPWD", oldpwd.to_str().unwrap().to_string());
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

fn cd_internal(opt_cmd_args: Option<&str>, env: &mut Env) -> bool {
	if opt_cmd_args.is_none() {
		let env_path = env.get("HOME");
		if env_path.is_none() {
			eprintln!("cd: HOME not set");
			false
		} else {
			let env_path = env.get("HOME").unwrap().clone();
			changedir(&env_path, env);
			true
		}
	} else if let Some(env_path) = env.get("HOME")
		&& opt_cmd_args.unwrap().as_bytes() == b"~"
	{
		changedir(&env_path.clone(), env)
	} else if opt_cmd_args.unwrap().as_bytes() == b"-" && env.get("OLDPWD").is_some() {
		let oldpwd = env.get("OLDPWD").unwrap().clone();
		changedir(&oldpwd, env)
	} else {
		changedir(opt_cmd_args.unwrap(), env)
	}
}

#[unsafe(no_mangle)]
pub unsafe fn builtin_cd(shell_env: &mut Env, cmd_opt: Option<*mut *const c_char>) -> i32 {
	if let Some(command) = cmd_opt {
		let cmd_args = *command.add(1);
		// option of cmd_args -> none if null
		let opt_cmd_args = if cmd_args.is_null() {
			None
		} else {
			Some(CStr::from_ptr(cmd_args).to_str().unwrap())
		};
		let status = cd_internal(opt_cmd_args, shell_env);
		arr_free(command as *mut *mut libc::c_char);
		return !status as i32;
	}
	1
}
