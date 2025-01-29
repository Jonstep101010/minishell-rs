use crate::prelude::*;

fn rm_prefix_redir_word_vec(args: &mut Vec<t_arg>) {
	let mut i = 0;
	while i < args.len() {
		if args[i].type_0 == REDIR_REMOVED && i + 1 < args.len() {
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
	// while i < arg.len() && !arg[i].elem.is_null() {
	while i < arg.len() {
		if arg[i].type_0 == REDIR {
			// unsafe {
			arg[i].elem_str = {
				if arg[i].redir == Some(INPUT_REDIR) || arg[i].redir == Some(OUTPUT_REDIR) {
					arg[i].elem_str[1..].to_string()
				} else {
					arg[i].elem_str[2..].to_string()
				}
			};
			// free_null(&mut arg[i].elem as *mut *mut libc::c_char as *mut libc::c_void);
			// }
		}
		i += 1;
	}
}

///
/// checks for a single token (piped command) if there are redirs contained
/// and processes those
pub fn process_redirections(token: &mut t_token) {
	let mut ii = 0;
	let mut redir: bool = false;
	let cmd_args = &mut token.cmd_args_vec;
	while ii < cmd_args.len() && !cmd_args[ii].elem_str.is_empty() {
		cmd_args[ii].redir = match cmd_args[ii].elem_str.as_str() {
			">>" => Some(APPEND),
			">" => Some(OUTPUT_REDIR),
			"<<" => Some(HEREDOC),
			"<" => Some(INPUT_REDIR),
			_ => cmd_args[ii].redir,
		};
		if cmd_args[ii].redir.is_some() {
			cmd_args[ii].type_0 = match cmd_args[ii].redir.unwrap() {
				APPEND | HEREDOC => {
					if (cmd_args[ii].elem_str).len() == 2 {
						REDIR_REMOVED
					} else {
						REDIR
					}
				}
				OUTPUT_REDIR | INPUT_REDIR => {
					if (cmd_args[ii].elem_str).len() == 1 {
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
		token.has_redir = true;
		parse_redir_types_vec(cmd_args);
		rm_prefix_redir_word_vec(cmd_args);
	}
}
