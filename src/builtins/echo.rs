use crate::prelude::*;
use crate::{t_shell, t_token};
use ::libc;
use libc::{printf, write};
use libft_rs::ft_strncmp::ft_strncmp;
use libutils_rs::src::array::arr_free::arr_free;
unsafe extern "C" fn is_n_arg(mut arg: *const libc::c_char) -> libc::c_int {
	if *arg as libc::c_int == '-' as i32 {
		arg = arg.offset(1);
		while *arg as libc::c_int == 'n' as i32 {
			arg = arg.offset(1);
			if *arg as libc::c_int == '\0' as i32 {
				return 1 as libc::c_int;
			}
		}
	}
	0 as libc::c_int
}
unsafe extern "C" fn echo_default(mut cmd_args: *const *const libc::c_char) {
	let mut i: libc::c_int = 0;
	let mut flag: libc::c_int = 0;
	while !(*cmd_args.offset(i as isize)).is_null()
		&& is_n_arg(*cmd_args.offset(i as isize)) == 1 as libc::c_int
	{
		i += 1;
	}
	if i == 0 as libc::c_int {
		flag = 2 as libc::c_int;
	}
	if i > 0 as libc::c_int {
		flag = 1 as libc::c_int;
	}
	let mut n_pos: libc::c_int = i;
	while !(*cmd_args.offset(i as isize)).is_null() {
		if flag == 1 as libc::c_int
			&& ft_strncmp(
				*cmd_args.offset(i as isize),
				b"-n\0" as *const u8 as *const libc::c_char,
				2 as libc::c_int as size_t,
			) == 0 as libc::c_int
			&& *(*cmd_args.offset(i as isize)).offset(2 as libc::c_int as isize) as libc::c_int
				!= '\0' as i32
			&& i != n_pos + 1 as libc::c_int
		{
			flag = 0 as libc::c_int;
		}
		printf(
			b"%s\0" as *const u8 as *const libc::c_char,
			*cmd_args.offset(i as isize),
		);
		if !(*cmd_args.offset((i + 1 as libc::c_int) as isize)).is_null() {
			printf(b" \0" as *const u8 as *const libc::c_char);
		}
		i += 1;
	}
	if flag != 1 as libc::c_int {
		printf(b"\n\0" as *const u8 as *const libc::c_char);
	}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn echo(mut _nullable: *mut t_shell, mut token: *mut t_token) -> libc::c_int {
	let mut args: *mut *const libc::c_char =
		crate::tokenizer::build_command::get_cmd_arr_token(token) as *mut *const libc::c_char;
	if !(*args.offset(1 as libc::c_int as isize)).is_null() {
		echo_default(&*args.offset(1 as libc::c_int as isize));
	} else {
		write(
			1 as libc::c_int,
			b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
			1 as libc::c_int as usize,
		);
	}
	arr_free(args as *mut *mut libc::c_char);
	0 as libc::c_int
}
