pub mod check_key;
pub mod expander;
pub mod get_env;
pub mod get_index;
use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug)]
pub struct Env {
	map: HashMap<String, String>,
}

impl Display for Env {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		for (key, value) in &self.map {
			write!(f, "{}={}\n", key, value)?;
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
	pub fn new() -> Self {
		// collect environment variables
		let mut host_env: HashMap<String, String> = std::env::vars().collect();
		// @note we will only expand the exit status when needed using shell
		// we panic if we can't get the current working directory
		assert!(host_env.get("PWD").is_some());
		if host_env.get("PATH").is_none() {
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
	// fn as_ptr_array(&self) -> Vec<*mut libc::c_char> {
	// 	let mut ptrs = Vec::new();
	// 	for (key, value) in &self.map {
	// 		let var: String = format!("{}={}", key, value);
	// 		ptrs.push(
	// 			(::std::ffi::CString::new(var))
	// 				.expect("Failed to convert environment variable into CString.")
	// 				.into_raw(),
	// 		);
	// 	}
	// 	ptrs.push(::core::ptr::null_mut());
	// 	ptrs
	// }
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
}

impl DerefMut for Env {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.map
	}
}
