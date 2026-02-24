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
	match (opt_target, env.get("HOME")) {
		(None, None) => {
			eprintln!("cd: HOME not set");
			false
		}
		(None, Some(env_path)) => {
			changedir(&env_path.clone(), env);
			true
		}
		(Some("~"), Some(env_path)) => changedir(&env_path.clone(), env),
		(Some("-"), _) if env.get("OLDPWD").is_some() => {
			let oldpwd = env.get("OLDPWD").unwrap().clone();
			changedir(&oldpwd, env)
		}
		(Some(target), _) => changedir(target, env),
	}
}

pub fn cd(shell_env: &mut Env, args: Vec<CString>) -> i32 {
	let opt_target_dir = if args.len() == 1 {
		None
	} else {
		Some(args[1].to_str().unwrap())
	};
	!cd_internal(opt_target_dir, shell_env) as i32
}
