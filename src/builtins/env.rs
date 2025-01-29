pub fn builtin_env(shell_env: &crate::Env) -> i32 {
	print!("{}", shell_env);
	println!("?={}", shell_env.get_status());
	0
}
