use ::libc;
use libft_rs::ft_strdup::ft_strdup;

use crate::{
	environment::{export_env::export_env, get_env::get_env},
	t_shell, t_token,
	tokenizer::build_command::get_cmd_arr_token,
};
use libutils_rs::src::{array::arr_free::arr_free, utils::free_mem::free_null};
use std::ffi::{CStr, CString};

fn changedir(mut rust_path: &str, mut shell: *mut t_shell) -> bool {
	let oldpwd = std::env::current_dir().unwrap();
	match std::env::set_current_dir(std::path::Path::new(rust_path)) {
		Ok(_) => {
			let pwd = std::env::current_dir();
			match pwd {
				Ok(p) => {
					let pwd_prefixed = String::from("PWD=") + p.to_str().unwrap();
					let old_prefixed = String::from("OLDPWD=") + oldpwd.to_str().unwrap();
					unsafe {
						export_env(
							shell,
							ft_strdup(CString::new(pwd_prefixed).unwrap().as_ptr()),
						);
						export_env(
							shell,
							ft_strdup(CString::new(old_prefixed).unwrap().as_ptr()),
						);
					}
					true
				}
				Err(e) => {
					eprintln!("cd: {}: {}", rust_path, e);
					false
				}
			}
		}
		Err(e) => {
			eprintln!("cd: {}: {}", rust_path, e);
			false
		}
	}
}
unsafe extern "C" fn cd_internal(mut opt_cmd_args: Option<&CStr>, mut shell: *mut t_shell) -> i32 {
	let mut path: *mut libc::c_char =
		get_env((*shell).env, b"HOME\0" as *const u8 as *const libc::c_char);
	let mut oldpwd: *mut libc::c_char = get_env(
		(*shell).env,
		b"OLDPWD\0" as *const u8 as *const libc::c_char,
	);
	if opt_cmd_args.is_none() && path.is_null() {
		free_null(&mut oldpwd as *mut *mut libc::c_char as *mut libc::c_void);
		eprintln!("cd: HOME not set");
		return 1;
	}
	if opt_cmd_args.is_none() && !path.is_null() {
		let strref = CStr::from_ptr(path).to_str().unwrap();
		changedir(strref, shell);
	} else if !path.is_null() && opt_cmd_args.unwrap().to_bytes() == b"~" {
		let strref = CStr::from_ptr(path).to_str().unwrap();
		println!("{}", strref);
		if !changedir(strref, shell) {
			libc::free(path as *mut libc::c_void);
			libc::free(oldpwd as *mut libc::c_void);
			return 1 as libc::c_int;
		}
	} else if opt_cmd_args.unwrap().to_bytes() == b"-" && !oldpwd.is_null() {
		let strref = CStr::from_ptr(oldpwd).to_str().unwrap();
		println!("{}", strref);
		if !changedir(strref, shell) {
			libc::free(path as *mut libc::c_void);
			libc::free(oldpwd as *mut libc::c_void);
			return 1 as libc::c_int;
		}
	} else if !changedir(opt_cmd_args.unwrap().to_str().unwrap(), shell) {
		libc::free(path as *mut libc::c_void);
		libc::free(oldpwd as *mut libc::c_void);
		return 1;
	}
	libc::free(path as *mut libc::c_void);
	libc::free(oldpwd as *mut libc::c_void);
	0
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
		Some(CStr::from_ptr(cmd_args))
	};
	let status = cd_internal(opt_cmd_args, shell);
	arr_free(command as *mut *mut libc::c_char);
	status
}
