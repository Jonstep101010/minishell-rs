use crate::prelude::*;

#[unsafe(no_mangle)]
pub unsafe fn builtin_env(shell_env: &mut Env, _command: Option<*mut *const c_char>) -> i32 {
	print!("{}", shell_env);
	// println!("?={}", shell.exit_status);// @note remove for now
	0
}
