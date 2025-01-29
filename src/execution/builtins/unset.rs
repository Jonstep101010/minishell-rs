fn check_illegal_char(str: &[u8]) -> bool {
	let mut i = 0;
	while str[i] != b'\0' {
		if b"?=;$.:><[]()/+-{}*#@!^".contains(&str[i]) {
			return true;
		}
		i += 1;
	}
	false
}

pub fn unset(shell_env: &mut crate::Env, args: Vec<crate::CString>) -> i32 {
	if args.len() == 1 || args[1].is_empty() {
		return 0;
	}
	let mut i = 1;
	while i < args.len() {
		let cur = args[i].to_str().unwrap();
		if check_illegal_char(args[i].as_bytes_with_nul())
			|| !crate::environment::check_valid_key(args[i].as_bytes_with_nul())
		{
			crate::eprint_msh!("unset: `{cur}': not a valid identifier");
			return 1;
		}
		if let Some(key) = shell_env.remove(cur) {
			println!("unset: {}", key);
		} else {
			crate::eprint_msh!("unset: `{}': not a valid identifier", cur);
			return 1;
		}
		i += 1;
	}
	0
}
