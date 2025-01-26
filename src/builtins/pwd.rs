use crate::prelude::*;

#[unsafe(no_mangle)]
pub unsafe fn builtin_pwd(shell_env: &mut Env, _cmd_opt: Option<*mut *const c_char>) -> i32 {
	// @note pwd in env should never be missing (init_shell will panic if it is)
	if let Some(pwd) = shell_env.get("PWD") {
		println!("{}", pwd);
	} else {
		let pwd = nix::unistd::getcwd().unwrap();
		println!("{}", pwd.display());
	}
	0
}
