use crate::{t_shell, t_token};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn builtin_pwd(
	mut shell: *mut t_shell,
	mut _token_nullable: *mut t_token,
) -> libc::c_int {
	// @note pwd in env should never be missing (init_shell will panic if it is)
	if let Some(pwd) = (*shell).env.get("PWD") {
		println!("{}", pwd);
	} else {
		let pwd = nix::unistd::getcwd().unwrap();
		println!("{}", pwd.display());
	}
	0 as libc::c_int
}
