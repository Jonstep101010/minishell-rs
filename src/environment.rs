pub mod check_key;
pub mod expander;
use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug)]
pub struct Env {
	map: HashMap<String, String>,
}

impl Display for Env {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		for (key, value) in &self.map {
			writeln!(f, "{}={}", key, value)?;
		}
		Ok(())
	}
}

use std::ops::{Deref, DerefMut};
impl Deref for Env {
	type Target = HashMap<String, String>;

	fn deref(&self) -> &Self::Target {
		&self.map
	}
}

impl Env {
	#[cfg(test)]
	pub fn new_empty() -> Self {
		Self {
			map: HashMap::new(),
		}
	}
	#[cfg(test)]
	pub fn new_exit_status() -> Self {
		let mut status_env = HashMap::new();
		status_env.insert("?".to_string(), "0".to_string());
		Self { map: status_env }
	}
	pub fn new() -> Self {
		// collect environment variables
		let mut host_env: HashMap<String, String> = std::env::vars().collect();
		// @note we will only expand the exit status when needed using shell
		// we panic if we can't get the current working directory
		assert!(host_env.contains_key("PWD"));
		if !host_env.contains_key("PATH") {
			host_env.insert(
				"PATH".to_string(),
				"/bin:/usr/bin:/sbin/:/usr/sbin".to_string(),
			);
		}
		Self { map: host_env }
	}
	pub fn get_paths(&self) -> Vec<String> {
		self.get("PATH")
			.unwrap()
			.split(':')
			.map(|s| s.to_string())
			.collect()
	}
	pub fn as_ptr_array(&self) -> Vec<*const i8> {
		let mut ptrs = Vec::new();
		for (key, value) in &self.map {
			let var: String = format!("{}={}", key, value);
			ptrs.push(var.as_ptr() as *const i8);
		}
		ptrs.push(::core::ptr::null_mut());
		ptrs
	}
	// fn destroy_ptr_array(ptrs: Vec<*mut libc::c_char>) {
	// 	for ptr in ptrs {
	// 		unsafe {
	// 			::std::ffi::CString::from_raw(ptr);
	// 		}
	// 	}
	// }
	// @todo implement builtins in environment
	pub fn export(&mut self, key: &str, value: String) {
		if key != "?" {
			self.map.insert(key.to_string(), value);
		}
	}
	// @todo implement builtins in environment
	pub fn unset(&mut self, key: &str) {
		self.map.remove(key);
	}
	pub fn get_slice(&self, key: &[u8]) -> Option<&String> {
		self.get(std::str::from_utf8(key).unwrap())
	}
}

impl Default for Env {
	fn default() -> Self {
		Self::new()
	}
}

impl DerefMut for Env {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.map
	}
}
