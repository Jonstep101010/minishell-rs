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

use crate::{environment::get_env::get_env, prelude::tcflag_t, t_shell, termios};

#[no_mangle]
pub unsafe extern "C" fn init_shell() -> *mut t_shell {
	let mut shell: *mut t_shell = ft_calloc(
		1 as libc::c_int as crate::size_t,
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
	(*shell).env = crate::environment::Env::new();
	shell
}
