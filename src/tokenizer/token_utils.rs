use ::libc;
use libft_rs::{ft_calloc::ft_calloc, ft_memcpy::ft_memcpy};
use libutils_rs::src::string::str_equal::equal;

use crate::{
	builtins::{
		cd::builtin_cd, echo::echo, env::builtin_env, exit::builtin_exit, export::builtin_export,
		pwd::builtin_pwd, unset::builtin_unset,
	},
	execution::exec_bin::exec_bin,
	prelude::*,
	size_t, t_arg, t_shell, t_token,
};

#[unsafe(no_mangle)]
pub unsafe fn set_cmd_func(mut cmd: *const libc::c_char, mut token: *mut t_token) {
	let cmds: [s_func; 8] = [
		{
			s_func {
				name: b"echo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(echo as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			}
		},
		{
			s_func {
				name: b"cd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(builtin_cd as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			}
		},
		{
			s_func {
				name: b"pwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(builtin_pwd as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			}
		},
		{
			s_func {
				name: b"export\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(builtin_export as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			}
		},
		{
			s_func {
				name: b"unset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(builtin_unset as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			}
		},
		{
			s_func {
				name: b"env\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(builtin_env as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			}
		},
		{
			s_func {
				name: b"exit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(builtin_exit as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			}
		},
		{
			s_func {
				name: std::ptr::null_mut::<libc::c_char>(),
				cmd: None,
			}
		},
	];
	let mut i: uint8_t = 0;
	while !(cmds[i as usize].name).is_null() {
		if !(equal(cmd, cmds[i as usize].name)).is_null() {
			(*token).cmd_func = cmds[i as usize].cmd;
			return;
		}
		i += 1;
	}
	(*token).cmd_func = Some(exec_bin as unsafe fn(&mut t_shell, *mut t_token) -> i32);
}
#[unsafe(no_mangle)]
pub unsafe fn init_cmdargs(mut size: size_t) -> *mut t_arg {
	let template: t_arg = {
		t_arg {
			elem: std::ptr::null_mut::<libc::c_char>(),
			type_0: e_arg::STRING,
			redir: e_redir::NO_REDIR,
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
			cmd_func: Some(exec_bin as unsafe fn(&mut t_shell, *mut t_token) -> i32),
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
