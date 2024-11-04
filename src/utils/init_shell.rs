use ::libc;
use libc::{exit, free, getcwd};
use libft_rs::{ft_calloc::ft_calloc, ft_strdup::ft_strdup};
use libutils_rs::src::{
	array::{
		append_str::{append_str_arr, append_str_arr_free},
		arr_free::arr_free,
	},
	string::join_strings::free_second_join,
	utils::free_mem::free_null,
};

use crate::{environment::get_env::get_env, t_shell, termios};

pub type size_t = libc::c_ulong;
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type e_redir = libc::c_uint;
pub const HEREDOC: e_redir = 4;
pub const APPEND: e_redir = 3;
pub const OUTPUT_REDIR: e_redir = 2;
pub const INPUT_REDIR: e_redir = 1;
pub const NO_REDIR: e_redir = 0;
pub type e_arg = libc::c_uint;
pub const REDIR_REMOVED: e_arg = 2;
pub const REDIR: e_arg = 1;
pub const STRING: e_arg = 0;
unsafe extern "C" fn init_env(mut envp: *const *mut libc::c_char) -> *mut *mut libc::c_char {
	let mut env: *mut *mut libc::c_char =
		append_str_arr(envp, b"?=0\0" as *const u8 as *const libc::c_char);
	let mut pwd: *mut libc::c_char = get_env(env, b"PWD\0" as *const u8 as *const libc::c_char);
	if pwd.is_null() && !env.is_null() {
		pwd = getcwd(std::ptr::null_mut::<libc::c_char>(), 0);
		if pwd.is_null() {
			arr_free(env);
			return std::ptr::null_mut::<libc::c_void>() as *mut *mut libc::c_char;
		}
		env = append_str_arr_free(
			append_str_arr_free(
				env,
				free_second_join(b"PWD=\0" as *const u8 as *const libc::c_char, pwd),
			),
			ft_strdup(b"OLDPWD=''\0" as *const u8 as *const libc::c_char),
		);
	} else {
		free_null(&mut pwd as *mut *mut libc::c_char as *mut libc::c_void);
	}
	let mut path: *mut libc::c_char = get_env(env, b"PATH\0" as *const u8 as *const libc::c_char);
	if path.is_null() {
		env = append_str_arr_free(
			env,
			ft_strdup(b"PATH=/bin:/usr/bin:/sbin/:/usr/sbin\0" as *const u8 as *const libc::c_char),
		);
	} else {
		free_null(&mut path as *mut *mut libc::c_char as *mut libc::c_void);
	}
	env
}
#[no_mangle]
pub unsafe extern "C" fn init_shell(mut envp: *const *mut libc::c_char) -> *mut t_shell {
	let mut shell: *mut t_shell = std::ptr::null_mut::<t_shell>();
	shell = ft_calloc(
		1 as libc::c_int as size_t,
		::core::mem::size_of::<t_shell>() as libc::c_ulong,
	) as *mut t_shell;
	if shell.is_null() {
		exit(1 as libc::c_int);
	}
	(*shell).p_termios = {
		termios {
			c_iflag: 0 as libc::c_int as tcflag_t,
			c_oflag: 0,
			c_cflag: 0,
			c_lflag: 0,
			c_line: 0,
			c_cc: [0; 32],
			c_ispeed: 0,
			c_ospeed: 0,
		}
	};
	(*shell).env = init_env(envp);
	if ((*shell).env).is_null() {
		free(shell as *mut libc::c_void);
		return std::ptr::null_mut::<t_shell>();
	}
	shell
}
