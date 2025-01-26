use ::libc;
use libc::free;
use libft_rs::{ft_strdup::ft_strdup, ft_strncmp::ft_strncmp};
use libutils_rs::src::utils::free_mem::free_null;

use crate::{prelude::*, size_t, t_arg};

fn rm_prefix_redir_word_vec(args: &mut Vec<t_arg>) {
	let mut i = 0;
	while i < args.len() {
		if args[i].type_0 == REDIR_REMOVED && i + 1 < args.len() {
			unsafe {
				free(args[i].elem as *mut libc::c_void);
			}
			args[i + 1].type_0 = REDIR;
			args[i + 1].redir = args[i].redir;
			args.remove(i);
			// Recursively call the function to handle the next element
			rm_prefix_redir_word_vec(args);
			return;
		}
		i += 1;
	}
}

fn parse_redir_types_vec(arg: &mut [t_arg]) {
	let mut i = 0;
	while i < arg.len() && !arg[i].elem.is_null() {
		if arg[i].type_0 == REDIR {
			unsafe {
				let tmp: *mut libc::c_char = {
					if arg[i].redir == Some(INPUT_REDIR) || arg[i].redir == Some(OUTPUT_REDIR) {
						ft_strdup(&*(arg[i].elem).add(1))
					} else {
						ft_strdup(&*(arg[i].elem).add(2))
					}
				};
				free_null(&mut arg[i].elem as *mut *mut libc::c_char as *mut libc::c_void);
				arg[i].elem = tmp;
			}
		}
		i += 1;
	}
}

#[unsafe(no_mangle)]
///
/// checks for a single token (piped command) if there are redirs contained
/// and processes those
pub unsafe fn process_redirections(token: *mut t_token) {
	let mut ii = 0;
	let mut redir: bool = false;
	let cmd_args = &mut (*token).cmd_args_vec;
	while ii < cmd_args.len() && !cmd_args[ii].elem.is_null() {
		cmd_args[ii].redir = match cmd_args[ii].elem {
			elem if ft_strncmp(elem, c">>".as_ptr(), 2 as size_t) == 0 => Some(APPEND),
			elem if ft_strncmp(elem, c">".as_ptr(), 1 as size_t) == 0 => Some(OUTPUT_REDIR),
			elem if ft_strncmp(elem, c"<<".as_ptr(), 2 as size_t) == 0 => Some(HEREDOC),
			elem if ft_strncmp(elem, c"<".as_ptr(), 1 as size_t) == 0 => Some(INPUT_REDIR),
			_ => cmd_args[ii].redir,
		};
		if cmd_args[ii].redir.is_some() {
			cmd_args[ii].type_0 = match cmd_args[ii].redir.unwrap() {
				APPEND | HEREDOC => {
					if *(cmd_args[ii].elem).add(2) == 0 {
						REDIR_REMOVED
					} else {
						REDIR
					}
				}
				OUTPUT_REDIR | INPUT_REDIR => {
					if *(cmd_args[ii].elem).add(1) == 0 {
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
		parse_redir_types_vec(cmd_args);
		rm_prefix_redir_word_vec(cmd_args);
	}
}
