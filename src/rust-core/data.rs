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
	pub fn new() -> Self {
		Self {
			vars: HashMap::new(),
		}
	}
	fn get(&self, key: &str) -> Option<&String> {
		self.vars.get(key)
	}

	fn set(&mut self, key: String, value: String) -> Option<String> {
		self.vars.insert(key, value)
	}

	fn remove(&mut self, key: &str) {
		self.vars.remove(key);
	}
	fn is_empty(&self) -> bool {
		self.vars.is_empty()
	}
}

#[cfg(test)]
mod tests_environment {
	use crate::data::Environment;

	#[test]
	fn test_env_new() {
		let env = Environment::new();
		assert!(env.vars.is_empty());
	}
	#[test]
	fn test_env_set() {
		let mut env = Environment::new();
		env.set("HOME".to_string(), "/home".to_string());
		assert_eq!(env.vars.len(), 1);
		assert_eq!(env.get("HOME"), Some(&"/home".to_string()));
	}
	#[test]
	fn test_env_remove() {
		let mut env = Environment::new();
		env.set("HOME".to_string(), "/home".to_string());
		env.remove("HOME");
		assert!(env.vars.is_empty());
	}
	#[test]
	fn test_env_is_empty_get_none() {
		let env = Environment::new();
		assert!(env.is_empty());
		assert_eq!(env.vars.len(), 0);
		assert_eq!(env.get("HOME"), None);
	}
	#[test]
	fn test_env_not_empty_get() {
		let mut env = Environment::new();
		env.set("HOME".to_string(), "/home".to_string());
		assert!(!env.is_empty());
		assert_eq!(env.get("HOME"), Some(&"/home".to_string()));
	}
	#[test]
	fn test_env_not_empty_default() {
		let env = Environment::default();
		assert!(!env.is_empty());
	}
	#[test]
	fn test_env_not_empty_default_update() {
		let mut env = Environment::default();
		assert_ne!(env.set("HOME".to_string(), "/home".to_string()), None);
		let mut env = Environment::default();
		let home = std::env::var("HOME").unwrap();
		assert_eq!(env.set("HOME".to_string(), "/home".to_string()), Some(home));
		assert!(!env.is_empty());
		let mut env = Environment::new();
		assert_eq!(env.set("HOME".to_string(), "/home".to_string()), None);
		assert!(!env.is_empty());
	}
}
impl Default for Environment {
	fn default() -> Self {
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
		Self { vars }
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

impl Default for Token {
	fn default() -> Self {
		Self::new()
	}
}

///
/// new tokens get created when new commands are entered
#[derive(Debug, Clone, Default)]
pub struct Shell {
	exit_status: i32,
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
	/// simple implementation: does not accept arguments
	pub fn exit(&self) -> ! {
		std::process::exit(self.exit_status);
	}

	///
	/// update the exit status of the shell
	fn update_exit_status(&mut self, status: i32) {
		self.exit_status = status;
	}

	///
	/// get the value of an environment variable
	pub fn get_env(&self, key: &str) -> Option<&String> {
		self.env.get(key)
	}

	fn echo(&self, line: &str) {
		if line.len() > 5 {
			println!("{}", &line[5..]);
		} else {
			println!();
		}
	}

	///
	/// not currently implemented
	fn exec(&mut self, line: &str) -> i32 {
		let mut parts = line.split_whitespace();
		if let Some(command) = parts.next() {
			let args: Vec<&str> = parts.collect();
			let status = std::process::Command::new(command)
				.args(&args)
				.status()
				.expect("failed to execute process");
			self.update_exit_status(status.code().unwrap_or(1));
			return status.code().unwrap_or(1);
		}
		1
	}

	pub fn process_line(&mut self, line: &str) {
		match line {
			"exit" => {
				self.exit();
			}
			"env" => {
				print!("{}", self.env);
			}
			"pwd" => {
				let pwd = self.get_env("PWD");
				println!("{}", pwd.unwrap());
			}
			_ => {
				// lexer: check syntax
				// expand variables
				// tokenize with pipes (not in quotes)
				// split tokens by whitespaces
				// execute
				// we do not care about pipes for now
				// cur line is a command and its arguments, this will be replaced by a token
				let cur_line = line.trim();
				if cur_line.starts_with("echo") {
					self.echo(cur_line);
				} else if cur_line.starts_with("export") {
					todo!("export");
				} else if cur_line.starts_with("unset") {
					todo!("unset");
				} else if cur_line.starts_with("cd") {
					todo!("cd");
				} else if self.exec(line) == 1 {
					println!("Command not found/error: {}", cur_line);
				}
			}
		}
	}
}

#[cfg(test)]
mod tests_shell {
	use super::*;

	#[test]
	fn test_shell_env() {
		let mut shell = Shell::new();
		shell.env.set("HOME".to_string(), "/home".to_string());
		assert_eq!(shell.get_env("HOME"), Some(&"/home".to_string()));
	}
	#[test]
	fn test_shell_env_unset() {
		let shell = Shell::new();
		assert!(shell.env.is_empty());
		assert_eq!(shell.get_env("HOME"), None);
	}
	#[test]
	fn test_shell_exit_status() {
		let mut shell = Shell::new();
		shell.update_exit_status(1);
		assert_eq!(shell.exit_status, 1);
	}
}
