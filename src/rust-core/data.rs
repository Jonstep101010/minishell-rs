#![allow(
	non_camel_case_types,
	// non_snake_case,
	// non_upper_case_globals,
	clippy::upper_case_acronyms
)]

use std::{
	collections::HashMap,
	fmt::{self, Formatter},
};

#[derive(Debug, Clone)]
pub struct Environment {
	pub vars: HashMap<String, String>,
}

impl Environment {
	///
	/// uses host environment variables
	fn new() -> Self {
		Environment {
			vars: {
				let mut vars = HashMap::new();
				for (key, value) in std::env::vars() {
					vars.insert(key, value);
				}
				if !vars.contains_key("PWD") {
					vars.insert(
						"PWD".to_string(),
						std::env::current_dir()
							.unwrap()
							.to_str()
							.unwrap()
							.to_string(),
					);
				}
				vars
			},
		}
	}
	fn get(&self, key: &str) -> Option<&String> {
		self.vars.get(key)
	}

	fn set(&mut self, key: String, value: String) {
		self.vars.insert(key, value);
	}

	fn remove(&mut self, key: &str) {
		self.vars.remove(key);
	}
}
impl fmt::Display for Environment {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		for (key, value) in &self.vars {
			writeln!(f, "{}: {}", key, value)?;
		}
		Ok(())
	}
}


// logics for redirections: something like this
// pub enum Option<T> {
//     None,
//     Some( /* … */ ),
// }
// first, we need to know if we have a redirection
// then, we need to know what type of redirection we have
// otherwise, we do not want to have to check
// the redirection type again

// try using option instead of this (e_type)
// #[derive(Debug, Clone)]
// pub enum ArgType {
//     String,
//     Redirection,
//     RemovedRedirection,
// }
#[derive(Debug, Clone)]
pub enum RedirType {
	Append,
	Output,
	Heredoc,
	Input,
}

#[derive(Debug, Clone)]
pub struct Command_Args {
	elem: String,
	arg_type: Option<RedirType>,
}

///
/// a token is a command with its arguments (split by pipes)
#[derive(Debug, Clone)]
pub struct Token {
	cmd_args: Vec<Command_Args>,
	// we could use std::process::Command instead of this
	bin: Option<String>,
	builtin_func: Option<fn(&mut Shell, &Token) -> u8>,
}

impl Token {
	pub fn new() -> Self {
		Token {
			cmd_args: Vec::new(),
			bin: None,
			builtin_func: None,
		}
	}
}

///
/// new tokens get created when new commands are entered
#[derive(Debug, Clone)]
pub struct Shell {
	exit_status: u8,
	pub env: Environment,
	tokens: Option<Vec<Token>>,
}

impl Shell {
	pub fn new() -> Self {
		Shell {
			exit_status: 0,
			env: Environment::new(),
			tokens: None,
		}
	}

	///
	/// update the exit status of the shell
	fn update_exit_status(&mut self, status: u8) {
		self.exit_status = status;
	}

	///
	/// get the value of an environment variable
	pub fn get_env(&self, key: &str) -> Option<&String> {
		self.env.get(key)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_shell_env() {
		let mut shell = Shell::new();
		shell.env.set("HOME".to_string(), "/home".to_string());
		assert_eq!(shell.get_env("HOME"), Some(&"/home".to_string()));
		assert_eq!(shell.env.get("HOME"), Some(&"/home".to_string()));
	}
	#[test]
	fn test_shell_exit_status() {
		let mut shell = Shell::new();
		shell.update_exit_status(1);
		assert_eq!(shell.exit_status, 1);
	}
}
