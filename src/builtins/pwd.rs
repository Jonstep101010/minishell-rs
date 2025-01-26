use crate::prelude::*;

pub fn builtin_pwd(shell_env: &Env) -> i32 {
	// @note pwd in env should never be missing (init_shell will panic if it is)
	if let Some(pwd) = shell_env.get("PWD") {
		println!("{}", pwd);
	} else {
		let pwd = nix::unistd::getcwd().unwrap();
		println!("{}", pwd.display());
	}
	0
}
