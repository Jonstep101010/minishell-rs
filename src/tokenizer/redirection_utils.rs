use ::libc;
use libc::free;
use libft_rs::{ft_strdup::ft_strdup, ft_strncmp::ft_strncmp};
use libutils_rs::src::utils::{free_mem::free_null, memsize::memsize};

use crate::{prelude::*, size_t, t_arg};
#[unsafe(no_mangle)]
pub unsafe fn rm_prefix_redir_word(mut arg: *mut t_arg) {
	if arg.is_null() {
		return;
	}
	let mut i = 0;
	let mut len = memsize(
		arg as *mut libc::c_void,
		::core::mem::size_of::<t_arg>() as libc::c_ulong,
	) as usize;
	while !((*arg.add(i)).elem).is_null() {
		if (*arg.add(i)).type_0 as libc::c_uint == e_arg::REDIR_REMOVED as i32 as libc::c_uint
			&& !((*arg.add(i + 1)).elem).is_null()
		{
			free((*arg.add(i)).elem as *mut libc::c_void);
			(*arg.add(i + 1)).type_0 = e_arg::REDIR;
			(*arg.add(i + 1)).redir = (*arg.add(i)).redir;
			while i < len {
				*arg.add(i) = *arg.add(i + 1);
				i += 1;
			}
			return rm_prefix_redir_word(arg);
		}
		i += 1;
	}
}
#[unsafe(no_mangle)]
pub unsafe fn parse_redir_types(mut arg: *mut t_arg) {
	let mut i = 0;
	loop {
		if ((*arg.add(i)).elem).is_null() {
			break;
		}
		if (*arg.add(i)).type_0 as libc::c_uint == e_arg::REDIR as i32 as libc::c_uint {
			let tmp: *mut libc::c_char = {
				if (*arg.add(i)).redir as libc::c_uint
					== e_redir::INPUT_REDIR as i32 as libc::c_uint
					|| (*arg.add(i)).redir as libc::c_uint
						== e_redir::OUTPUT_REDIR as i32 as libc::c_uint
				{
					ft_strdup(&*((*arg.add(i)).elem).add(1))
				} else {
					ft_strdup(&*((*arg.add(i)).elem).add(2))
				}
			};
			free_null(&mut (*arg.add(i)).elem as *mut *mut libc::c_char as *mut libc::c_void);
			let fresh0 = &mut (*arg.add(i)).elem;
			*fresh0 = tmp;
		}
		i += 1;
	}
}
unsafe fn set_type_redir(mut cmd_arg: *mut t_arg) {
	if (*cmd_arg).redir as libc::c_uint == e_redir::APPEND as i32 as libc::c_uint
		|| (*cmd_arg).redir as libc::c_uint == e_redir::HEREDOC as i32 as libc::c_uint
	{
		if *((*cmd_arg).elem).add(2) == 0 {
			(*cmd_arg).type_0 = e_arg::REDIR_REMOVED;
		} else {
			(*cmd_arg).type_0 = e_arg::REDIR;
		}
	} else if (*cmd_arg).redir as libc::c_uint == e_redir::OUTPUT_REDIR as i32 as libc::c_uint
		|| (*cmd_arg).redir as libc::c_uint == e_redir::INPUT_REDIR as i32 as libc::c_uint
	{
		if *((*cmd_arg).elem).add(1) == 0 {
			(*cmd_arg).type_0 = e_arg::REDIR_REMOVED;
		} else {
			(*cmd_arg).type_0 = e_arg::REDIR;
		}
	}
}
#[unsafe(no_mangle)]
pub unsafe fn check_redirections(mut cmd_args: *mut t_arg) -> bool {
	let mut ii = 0;
	let mut redir: bool = false;
	while !((*cmd_args.add(ii)).elem).is_null() {
		if ft_strncmp(
			(*cmd_args.add(ii)).elem,
			b">>\0" as *const u8 as *const libc::c_char,
			2 as size_t,
		) == 0
		{
			(*cmd_args.add(ii)).redir = e_redir::APPEND;
		} else if ft_strncmp(
			(*cmd_args.add(ii)).elem,
			b">\0" as *const u8 as *const libc::c_char,
			1 as size_t,
		) == 0
		{
			(*cmd_args.add(ii)).redir = e_redir::OUTPUT_REDIR;
		} else if ft_strncmp(
			(*cmd_args.add(ii)).elem,
			b"<<\0" as *const u8 as *const libc::c_char,
			2 as size_t,
		) == 0
		{
			(*cmd_args.add(ii)).redir = e_redir::HEREDOC;
		} else if ft_strncmp(
			(*cmd_args.add(ii)).elem,
			b"<\0" as *const u8 as *const libc::c_char,
			1 as size_t,
		) == 0
		{
			(*cmd_args.add(ii)).redir = e_redir::INPUT_REDIR;
		}
		if (*cmd_args.add(ii)).redir as libc::c_uint != e_redir::NO_REDIR as i32 as libc::c_uint {
			set_type_redir(&mut *cmd_args.add(ii));
			redir = 1 != 0;
		}
		ii += 1;
	}
	redir
}
