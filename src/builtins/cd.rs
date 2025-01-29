use crate::{CString, Env};
use std::path::Path;

fn changedir(path_string: &str, env: &mut Env) -> bool {
	let oldpwd = std::env::current_dir().unwrap();
	let rust_path = Path::new(path_string);
	match std::env::set_current_dir(rust_path) {
		Ok(_) => {
			let pwd = std::env::current_dir();
			match pwd {
				Ok(p) => {
					env.export("PWD", p.to_str().unwrap().to_string());
					env.export("OLDPWD", oldpwd.to_str().unwrap().to_string());
					true
				}
				Err(e) => {
					eprintln!("cd: {}: {}", path_string, e);
					false
				}
			}
		}
		Err(e) => {
			eprintln!("cd: {}: {}", path_string, e);
			false
		}
	}
}

fn cd_internal(opt_target: Option<&str>, env: &mut Env) -> bool {
	if opt_target.is_none() {
		let env_path = env.get("HOME");
		if env_path.is_none() {
			eprintln!("cd: HOME not set");
			false
		} else {
			let env_path = env.get("HOME").unwrap().clone();
			changedir(&env_path, env);
			true
		}
	} else if let Some(env_path) = env.get("HOME")
		&& opt_target.unwrap().as_bytes() == b"~"
	{
		changedir(&env_path.clone(), env)
	} else if opt_target.unwrap().as_bytes() == b"-" && env.get("OLDPWD").is_some() {
		let oldpwd = env.get("OLDPWD").unwrap().clone();
		changedir(&oldpwd, env)
	} else {
		changedir(opt_target.unwrap(), env)
	}
}

pub fn builtin_cd(shell_env: &mut Env, args: Vec<CString>) -> i32 {
	let opt_target_dir = if args.len() == 1 {
		None
	} else {
		Some(args[1].to_str().unwrap())
	};
	!cd_internal(opt_target_dir, shell_env) as i32
}
