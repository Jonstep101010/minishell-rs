use ::libc;
use libc::free;
use libft_rs::{ft_strdup::ft_strdup, ft_strncmp::ft_strncmp};
use libutils_rs::src::utils::{free_mem::free_null, memsize::memsize};

use crate::{prelude::*, size_t, t_arg};
#[unsafe(no_mangle)]
unsafe fn rm_prefix_redir_word(arg: *mut t_arg) {
	let mut i = 0;
	let mut len = memsize(
		arg as *mut libc::c_void,
		::core::mem::size_of::<t_arg>() as libc::c_ulong,
	) as usize;
	while !((*arg.add(i)).elem).is_null() {
		if (*arg.add(i)).type_0 == REDIR_REMOVED && !((*arg.add(i + 1)).elem).is_null() {
			free((*arg.add(i)).elem as *mut libc::c_void);
			(*arg.add(i + 1)).type_0 = REDIR;
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
unsafe fn parse_redir_types(mut arg: *mut t_arg) {
	let mut i = 0;
	loop {
		if ((*arg.add(i)).elem).is_null() {
			break;
		}
		if (*arg.add(i)).type_0 == REDIR {
			let tmp: *mut libc::c_char = {
				if (*arg.add(i)).redir == Some(INPUT_REDIR)
					|| (*arg.add(i)).redir == Some(OUTPUT_REDIR)
				{
					ft_strdup(&*((*arg.add(i)).elem).add(1))
				} else {
					ft_strdup(&*((*arg.add(i)).elem).add(2))
				}
			};
			free_null(&mut (*arg.add(i)).elem as *mut *mut libc::c_char as *mut libc::c_void);
			(*arg.add(i)).elem = tmp;
		}
		i += 1;
	}
}

#[unsafe(no_mangle)]
///
/// checks for a single token (piped command) if there are redirs contained
/// and processes those
pub unsafe fn process_redirections(mut token: *mut t_token) {
	let mut ii = 0;
	let mut redir: bool = false;
	let mut cmd_args = (*token).cmd_args;
	while !((*cmd_args.add(ii)).elem).is_null() {
		(*cmd_args.add(ii)).redir = match (*cmd_args.add(ii)).elem {
			elem if ft_strncmp(elem, c">>".as_ptr(), 2 as size_t) == 0 => Some(APPEND),
			elem if ft_strncmp(elem, c">".as_ptr(), 1 as size_t) == 0 => Some(OUTPUT_REDIR),
			elem if ft_strncmp(elem, c"<<".as_ptr(), 2 as size_t) == 0 => Some(HEREDOC),
			elem if ft_strncmp(elem, c"<".as_ptr(), 1 as size_t) == 0 => Some(INPUT_REDIR),
			_ => (*cmd_args.add(ii)).redir,
		};
		if (*cmd_args.add(ii)).redir.is_some() {
			(*(cmd_args.add(ii))).type_0 = match (*(cmd_args.add(ii))).redir.unwrap() {
				APPEND | HEREDOC => {
					if *((*(cmd_args.add(ii))).elem).add(2) == 0 {
						REDIR_REMOVED
					} else {
						REDIR
					}
				}
				OUTPUT_REDIR | INPUT_REDIR => {
					if *((*(cmd_args.add(ii))).elem).add(1) == 0 {
						REDIR_REMOVED
					} else {
						REDIR
					}
				}
			};
			redir = true;
		}
		ii += 1;
	}
	if redir {
		(*token).has_redir = true;
		parse_redir_types((*token).cmd_args);
		rm_prefix_redir_word((*token).cmd_args);
	}
}
