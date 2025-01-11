use crate::{environment::Env, prelude::*};
use nix::unistd::*;
use std::ffi::{CStr, CString};

fn find_bin(bin_prefix: &str, paths: Vec<String>) -> Option<CString> {
	for path_prefix in paths {
		let bin_path = format!("{}{}", path_prefix, bin_prefix);
		if nix::unistd::access(bin_path.as_str(), AccessFlags::F_OK).is_ok() {
			return Some(CString::new(bin_path).unwrap());
		}
	}
	None
}

fn get_bin(bin: &CStr, paths: Vec<String>) -> (u8, Option<CString>) {
	// prefix bin with /: format will panic if input invalid
	let bin_prefix = format!("{}{}", "/", bin.to_str().unwrap());
	match find_bin(&bin_prefix, paths) {
		Some(bin_path) => {
			if nix::unistd::access(bin_path.as_c_str(), AccessFlags::X_OK).is_err() {
				(126, None)
			} else {
				(0, Some(bin_path))
			}
		}
		_ => (127, None),
	}
}

fn set_single(bin: &CStr, env: &Env) -> (u8, Option<CString>) {
	match bin.to_bytes()[0] {
		b'/' => {
			eprint_msh!("/: Is a directory");
			(126, None)
		}
		b'.' => {
			eprint_msh!(".: filename argument required ");
			(2, None)
		}
		_ => {
			if let Some(home_path) = env.get("HOME") {
				eprint_msh!("{}: Is a directory", home_path);
			} else {
				eprint_msh!("/home/minishell: Is a directory");
			}
			(126, None)
		}
	}
}

pub fn get_path_prefixed(env: &Env, bin: &CStr) -> (u8, Option<CString>) {
	if bin.is_empty() {
		return (127, None);
	}
	let bin_bytes = bin.to_bytes();
	if bin_bytes.len() == 1 {
		if bin_bytes
			.iter()
			.any(|&c| c == b'/' || c == b'.' || c == b'~' || c == b'\0')
		{
			return set_single(bin, env);
		}
		if bin_bytes[0] == b'.' || bin_bytes[0] == b'/' {
			let binpath = CString::from(bin);
			if bin_bytes.len() == 2 && bin_bytes == b".." {
				return (127, None);
			}
			if binpath.is_empty() {
				return (1, None);
			}
			if nix::unistd::access(binpath.as_c_str(), AccessFlags::F_OK).is_err() {
				return (127, None);
			}
			if nix::unistd::access(binpath.as_c_str(), AccessFlags::X_OK).is_err() {
				return (126, None);
			}
			return (0, Some(binpath));
		}
	}
	get_bin(bin, env.get_paths())
}
