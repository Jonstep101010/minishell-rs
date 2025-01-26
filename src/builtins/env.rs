use crate::prelude::*;

pub fn builtin_env(shell_env: &Env) -> i32 {
	print!("{}", shell_env);
	println!("?={}", shell_env.get_status());
	0
}
