use ::libc;
use libc::free;
use libft_rs::{ft_strdup::ft_strdup, ft_strncmp::ft_strncmp};
use libutils_rs::src::utils::{free_mem::free_null, memsize::memsize};

use crate::{prelude::*, size_t, t_arg};
#[no_mangle]
pub unsafe extern "C" fn rm_prefix_redir_word(mut arg: *mut t_arg) {
	if arg.is_null() {
		return;
	}
	let mut i: size_t = 0;
	let mut len: size_t = memsize(
		arg as *mut libc::c_void,
		::core::mem::size_of::<t_arg>() as libc::c_ulong,
	);
	while !((*arg.offset(i as isize)).elem).is_null() {
		if (*arg.offset(i as isize)).type_0 as libc::c_uint
			== REDIR_REMOVED as libc::c_int as libc::c_uint
			&& !((*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)).elem)
				.is_null()
		{
			free((*arg.offset(i as isize)).elem as *mut libc::c_void);
			(*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)).type_0 =
				REDIR;
			(*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)).redir =
				(*arg.offset(i as isize)).redir;
			while i < len {
				*arg.offset(i as isize) =
					*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
				i = i.wrapping_add(1);
			}
			return rm_prefix_redir_word(arg);
		}
		i = i.wrapping_add(1);
	}
}
#[no_mangle]
pub unsafe extern "C" fn parse_redir_types(mut arg: *mut t_arg) {
	let mut i: libc::c_int = -1;
	let mut tmp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	loop {
		i += 1;
		if ((*arg.offset(i as isize)).elem).is_null() {
			break;
		}
		if (*arg.offset(i as isize)).type_0 as libc::c_uint == REDIR as libc::c_int as libc::c_uint
		{
			if (*arg.offset(i as isize)).redir as libc::c_uint
				== INPUT_REDIR as libc::c_int as libc::c_uint
				|| (*arg.offset(i as isize)).redir as libc::c_uint
					== OUTPUT_REDIR as libc::c_int as libc::c_uint
			{
				tmp =
					ft_strdup(&*((*arg.offset(i as isize)).elem).offset(1 as libc::c_int as isize));
			} else {
				tmp =
					ft_strdup(&*((*arg.offset(i as isize)).elem).offset(2 as libc::c_int as isize));
			}
			free_null(
				&mut (*arg.offset(i as isize)).elem as *mut *mut libc::c_char as *mut libc::c_void,
			);
			let fresh0 = &mut (*arg.offset(i as isize)).elem;
			*fresh0 = tmp;
		}
	}
}
unsafe extern "C" fn set_type_redir(mut cmd_arg: *mut t_arg) {
	if (*cmd_arg).redir as libc::c_uint == APPEND as libc::c_int as libc::c_uint
		|| (*cmd_arg).redir as libc::c_uint == HEREDOC as libc::c_int as libc::c_uint
	{
		if *((*cmd_arg).elem).offset(2 as libc::c_int as isize) == 0 {
			(*cmd_arg).type_0 = REDIR_REMOVED;
		} else {
			(*cmd_arg).type_0 = REDIR;
		}
	} else if (*cmd_arg).redir as libc::c_uint == OUTPUT_REDIR as libc::c_int as libc::c_uint
		|| (*cmd_arg).redir as libc::c_uint == INPUT_REDIR as libc::c_int as libc::c_uint
	{
		if *((*cmd_arg).elem).offset(1 as libc::c_int as isize) == 0 {
			(*cmd_arg).type_0 = REDIR_REMOVED;
		} else {
			(*cmd_arg).type_0 = REDIR;
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn check_redirections(mut cmd_args: *mut t_arg) -> e_redir {
	let mut ii: size_t = 0;
	let mut redir: bool = false;
	while !((*cmd_args.offset(ii as isize)).elem).is_null() {
		if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b">>\0" as *const u8 as *const libc::c_char,
			2 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = APPEND;
		} else if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b">\0" as *const u8 as *const libc::c_char,
			1 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = OUTPUT_REDIR;
		} else if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b"<<\0" as *const u8 as *const libc::c_char,
			2 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = HEREDOC;
		} else if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b"<\0" as *const u8 as *const libc::c_char,
			1 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = INPUT_REDIR;
		}
		if (*cmd_args.offset(ii as isize)).redir as libc::c_uint
			!= NO_REDIR as libc::c_int as libc::c_uint
		{
			set_type_redir(&mut *cmd_args.offset(ii as isize));
			redir = 1 as libc::c_int != 0;
		}
		ii = ii.wrapping_add(1);
	}
	redir as e_redir
}
