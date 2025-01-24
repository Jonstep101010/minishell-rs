use crate::{t_shell, t_token};

#[allow(unused_mut)]
#[unsafe(no_mangle)]
pub unsafe fn builtin_env(mut shell: &mut t_shell, mut _token: *mut t_token) -> i32 {
	print!("{}", shell.env);
	println!("?={}", shell.exit_status);
	0
}
