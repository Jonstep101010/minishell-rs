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

pub type t_cmd_func_builtin =
	Option<unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_func {
	pub name: *mut libc::c_char,
	pub cmd: t_cmd_func_builtin,
}
#[no_mangle]
pub unsafe extern "C" fn set_cmd_func(mut cmd: *const libc::c_char, mut token: *mut t_token) {
	let mut i: uint8_t = 0;
	let cmds: [s_func; 8] = [
		{
			s_func {
				name: b"echo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(echo as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int),
			}
		},
		{
			s_func {
				name: b"cd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(
					builtin_cd as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
				),
			}
		},
		{
			s_func {
				name: b"pwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(
					builtin_pwd as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
				),
			}
		},
		{
			s_func {
				name: b"export\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(
					builtin_export
						as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
				),
			}
		},
		{
			s_func {
				name: b"unset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(
					builtin_unset
						as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
				),
			}
		},
		{
			s_func {
				name: b"env\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(
					builtin_env as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
				),
			}
		},
		{
			s_func {
				name: b"exit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
				cmd: Some(
					builtin_exit as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
				),
			}
		},
		{
			s_func {
				name: std::ptr::null_mut::<libc::c_char>(),
				cmd: None,
			}
		},
	];
	i = 0 as libc::c_int as uint8_t;
	while !(cmds[i as usize].name).is_null() {
		if !(equal(cmd, cmds[i as usize].name)).is_null() {
			(*token).cmd_func = cmds[i as usize].cmd;
			return;
		}
		i = i.wrapping_add(1);
	}
	(*token).cmd_func =
		Some(exec_bin as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn init_cmdargs(mut size: size_t) -> *mut t_arg {
	let mut args: *mut t_arg = std::ptr::null_mut::<t_arg>();
	let template: t_arg = {
		t_arg {
			elem: std::ptr::null_mut::<libc::c_char>(),
			type_0: STRING,
			redir: NO_REDIR,
		}
	};
	args = ft_calloc(
		size.wrapping_add(1 as libc::c_int as libc::c_ulong),
		::core::mem::size_of::<t_arg>() as libc::c_ulong,
	) as *mut t_arg;
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
#[no_mangle]
pub unsafe extern "C" fn init_token(mut size: size_t) -> *mut t_token {
	let mut token: *mut t_token = std::ptr::null_mut::<t_token>();
	let template: t_token = {
		t_token {
			cmd_args: std::ptr::null_mut::<t_arg>(),
			has_redir: 0 as libc::c_int != 0,
			split_pipes: std::ptr::null_mut::<libc::c_char>(),
			tmp_arr: std::ptr::null_mut::<*mut libc::c_char>(),
			bin: std::ptr::null_mut::<libc::c_char>(),
			cmd_func: Some(
				exec_bin as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
			),
		}
	};
	token = ft_calloc(
		size.wrapping_add(1 as libc::c_int as libc::c_ulong),
		::core::mem::size_of::<t_token>() as libc::c_ulong,
	) as *mut t_token;
	while !token.is_null() && {
		let fresh1 = size;
		size = size.wrapping_sub(1);
		fresh1 != 0
	} {
		ft_memcpy(
			&mut *token.offset(size as isize) as *mut t_token as *mut libc::c_void,
			&template as *const t_token as *const libc::c_void,
			::core::mem::size_of::<t_token>() as libc::c_ulong,
		);
	}
	token
}
