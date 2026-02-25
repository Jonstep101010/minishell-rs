use crate::msh::{
	ArgType::{Redir, RedirRemoved},
	CommandArg, CommandToken,
	RedirType::{Append, HereDoc, InputRedir, OutputRedir},
};

fn rm_prefix_redir_word_vec(args: &mut Vec<CommandArg>) {
	let mut i = 0;
	while i < args.len() {
		if args[i].type_0 == Some(RedirRemoved) && i + 1 < args.len() {
			args[i + 1].type_0 = Some(Redir);
			args[i + 1].redir = args[i].redir;
			args.remove(i);
			// Recursively call the function to handle the next element
			rm_prefix_redir_word_vec(args);
			return;
		}
		i += 1;
	}
}

fn parse_redir_types_vec(arg: &mut [CommandArg]) {
	for item in arg {
		if item.type_0 == Some(Redir) {
			item.elem_str = {
				if item.redir == Some(InputRedir) || item.redir == Some(OutputRedir) {
					item.elem_str[1..].to_string()
				} else {
					item.elem_str[2..].to_string()
				}
			};
		}
	}
}

impl CommandToken {
	///
	/// checks for a single token (piped command) if there are redirs contained
	/// and processes those
	pub(super) fn process_redirections(&mut self) {
		let mut ii = 0;
		let cmd_args = &mut self.cmd_args_vec;
		while ii < cmd_args.len() && !cmd_args[ii].elem_str.is_empty() {
			cmd_args[ii].redir = match cmd_args[ii].elem_str.as_str() {
				">>" => Some(Append),
				">" => Some(OutputRedir),
				"<<" => Some(HereDoc),
				"<" => Some(InputRedir),
				_ => cmd_args[ii].redir,
			};
			if let Some(redirtype) = cmd_args[ii].redir {
				cmd_args[ii].type_0 = Some(match redirtype {
					Append | HereDoc if (cmd_args[ii].elem_str).len() == 2 => RedirRemoved,
					OutputRedir | InputRedir if (cmd_args[ii].elem_str).len() == 1 => RedirRemoved,
					_ => Redir,
				});
				self.has_redir = true;
			}
			ii += 1;
		}
		if self.has_redir {
			parse_redir_types_vec(cmd_args);
			rm_prefix_redir_word_vec(cmd_args);
		}
	}
}
