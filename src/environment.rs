pub mod check_key;
pub mod expander;
use crate::prelude::*;
use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug)]
pub struct Env {
	map: HashMap<String, String>,
	status: i32,
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
			status: 0,
		}
	}
	#[cfg(test)]
	pub fn new_exit_status() -> Self {
		let mut status_env = HashMap::new();
		status_env.insert("?".to_string(), "0".to_string());
		Self {
			map: status_env,
			status: 0,
		}
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
		Self {
			map: host_env,
			status: 0,
		}
	}
	#[cfg(test)]
	pub fn new_test() -> Self {
		// collect environment variables
		#[cfg(miri)]
		{
			let mut host_env: HashMap<String, String> = HashMap::new();
			host_env.insert("USER".to_string(), "someweirdname".to_string());
			Self {
				map: host_env,
				status: 0,
			}
		}
		#[cfg(not(miri))]
		{
			Self::new()
		}
	}
	pub fn get_paths(&self) -> Vec<String> {
		self.get("PATH")
			.unwrap()
			.split(':')
			.map(|s| s.to_string())
			.collect()
	}
	// pub fn as_ptr_array(&self) -> Vec<*const i8> {
	// 	let mut ptrs = Vec::new();
	// 	for (key, value) in &self.map {
	// 		let var: String = format!("{}={}", key, value);
	// 		ptrs.push(var.as_ptr() as *const i8);
	// 	}
	// 	ptrs.push(::core::ptr::null_mut());
	// 	ptrs
	// }
	pub fn to_cstring_vec(&self) -> Vec<CString> {
		let mut vec_cstrings = Vec::new();
		for (key, value) in &self.map {
			let var = CString::new(format!("{}={}", key, value)).unwrap();
			vec_cstrings.push(var);
		}
		vec_cstrings
	}
	// @todo implement builtins in environment
	pub fn export(&mut self, key: &str, value: String) {
		if key != "?" {
			self.map.insert(key.to_string(), value);
		}
	}
	pub fn set_status(&mut self, new_status: i32) {
		self.status = new_status;
	}
	pub fn get_status(&self) -> i32 {
		self.status
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
