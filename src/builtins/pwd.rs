pub fn builtin_pwd(shell_env: &crate::Env) -> i32 {
	if let Some(pwd) = shell_env.get("PWD") {
		println!("{}", pwd);
	} else {
		let pwd = nix::unistd::getcwd().unwrap();
		println!("{}", pwd.display());
	}
	0
}
