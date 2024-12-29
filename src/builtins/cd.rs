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
						let new_pwd = CString::new(pwd_prefixed).unwrap();
						export_env(shell, ft_strdup(new_pwd.as_ptr()));
						let old_pwd = CString::new(old_prefixed).unwrap();
						export_env(shell, ft_strdup(old_pwd.as_ptr()));
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
unsafe fn cd_internal(mut opt_cmd_args: Option<&CStr>, mut shell: *mut t_shell) -> i32 {
	let mut path = get_env((*shell).env, c"HOME".as_ptr());
	let mut oldpwd = get_env((*shell).env, c"OLDPWD".as_ptr());
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
