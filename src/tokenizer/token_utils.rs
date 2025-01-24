use ::libc;
use libft_rs::{ft_calloc::ft_calloc, ft_memcpy::ft_memcpy};

use crate::{prelude::*, size_t, t_arg, t_token};

#[unsafe(no_mangle)]
pub unsafe fn init_cmdargs(mut size: size_t) -> *mut t_arg {
	let template: t_arg = {
		t_arg {
			elem: std::ptr::null_mut::<libc::c_char>(),
			type_0: e_arg::STRING,
			redir: None,
		}
	};
	let mut args: *mut t_arg =
		ft_calloc(size + 1, ::core::mem::size_of::<t_arg>() as libc::c_ulong) as *mut t_arg;
	while !args.is_null() && {
		let fresh0 = size;
		size = size.wrapping_sub(1);
		fresh0 != 0
	} {
		ft_memcpy(
			&mut *args.offset(size as isize) as *mut t_arg as *mut libc::c_void,
			&template as *const t_arg as *const libc::c_void,
			::core::mem::size_of::<t_arg>() as libc::c_ulong,
		);
	}
	args
}

// cstring causes issues in dbg
// pub unsafe fn init_cmdargs_vec(size: usize) -> Vec<t_arg> {
// 	vec![
// 		t_arg {
// 			elem: CString::new("").unwrap(),
// 			type_0: e_arg::STRING,
// 			redir: e_redir::NO_REDIR,
// 		};
// 		size
// 	]
// }

#[unsafe(no_mangle)]
pub unsafe fn init_token(mut size: size_t) -> *mut t_token {
	let template: t_token = {
		t_token {
			cmd_args: std::ptr::null_mut::<t_arg>(),
			has_redir: false,
			split_pipes: std::ptr::null_mut::<libc::c_char>(),
			tmp_arr: std::ptr::null_mut::<*mut libc::c_char>(),
			bin: std::ffi::CString::new("").unwrap(),
			cmd_func: None,
			split_non_quoted: String::new(),
		}
	};
	let mut token: *mut t_token =
		ft_calloc(size + 1, ::core::mem::size_of::<t_token>() as libc::c_ulong) as *mut t_token;
	while !token.is_null() && {
		let fresh1 = size;
		size = size.wrapping_sub(1);
		fresh1 != 0
	} {
		ft_memcpy(
			&mut *token.add(size as usize) as *mut t_token as *mut libc::c_void,
			&template as *const t_token as *const libc::c_void,
			::core::mem::size_of::<t_token>() as libc::c_ulong,
		);
	}
	token
}
