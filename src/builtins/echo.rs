fn is_n_arg(arg: &[u8]) -> bool {
	if arg[0] == b'-' {
		let mut i = 1;
		while arg[i] == b'n' {
			i += 1;
			if arg[i] == b'\0' {
				return true;
			}
		}
	}
	false
}

pub fn echo(args: Vec<crate::CString>) -> i32 {
	if args.len() != 1 {
		let cmd_args: &[crate::CString] = &args[1..];
		let mut i = 0;
		while i < cmd_args.len() && is_n_arg(cmd_args[i].as_bytes_with_nul()) {
			i += 1;
		}
		let mut flag = match i {
			0 => 2,
			_ => 1,
		};
		let n_pos = i;
		while i < cmd_args.len() {
			if flag == 1
				&& cmd_args[i].count_bytes() == 2
				&& cmd_args[i].as_bytes() == b"-n"
				&& i != n_pos + 1
			{
				flag = 0;
			}
			print!(
				"{}",
				cmd_args[i]
					.to_str()
					.expect("CStr::from_bytes_with_nul failed")
			);
			if i + 1 < cmd_args.len() {
				print!(" ");
			}
			i += 1;
		}
		if flag != 1 {
			println!();
		}
	} else {
		println!();
	}
	0
}
