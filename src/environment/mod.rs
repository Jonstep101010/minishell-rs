#![warn(clippy::pedantic)]

mod tests_expander;
use std::{collections::HashMap, ffi::CString, fmt::Display};

pub fn check_valid_key(s: &[u8]) -> bool {
	let mut i = 0;
	if s[0] == b'=' || !(matches!(s[0] as char,'A'..='Z' | 'a'..='z' | '_')) {
		return false;
	}
	while s[i] != 0 && s[i] != b'=' {
		if matches!(s[i] as char,'A'..='Z' | 'a'..='z' | '0'..='9' | '_') {
			i += 1;
		} else {
			return false;
		}
	}
	!(s[i] == b'=' && s[i + 1] == b'=')
}

#[derive(Clone, Debug)]
pub struct Env {
	map: HashMap<String, String>,
	status: i32,
}

impl Display for Env {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		for (key, value) in &self.map {
			writeln!(f, "{key}={value}")?;
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
			.map(std::string::ToString::to_string)
			.collect()
	}
	pub fn to_cstring_vec(&self) -> Vec<CString> {
		let mut vec_cstrings = Vec::new();
		for (key, value) in &self.map {
			let var = CString::new(format!("{key}={value}")).unwrap();
			vec_cstrings.push(var);
		}
		vec_cstrings
	}
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
	pub fn get_slice(&self, key: &[u8]) -> Option<&String> {
		self.get(std::str::from_utf8(key).unwrap())
	}
	/// Expand the input string using the environment
	pub fn expander(&self, input_expander: &mut String) {
		const CHARMATCH: &[u8; 8] = b"$\"'/? )(";
		if !input_expander.contains('$') {
			return;
		}
		let mut i = 0;
		let mut should_expand = true;
		let mut has_double_quote = false;
		let mut ret = String::new();
		let bytes = input_expander.as_bytes();
		let idx_advance = |bytes_at_i: &[u8]| {
			let mut count: usize = 0;
			while count + 1 < bytes_at_i.len()
				&& !CHARMATCH.iter().any(|&x| x == bytes_at_i[count + 1])
			{
				count += 1;
			}
			if count < bytes_at_i.len() - 1 && bytes_at_i[count + 1] == b'?' {
				count += 1;
			}
			count
		};
		while i < bytes.len() {
			if bytes[i] == b'"' {
				has_double_quote = !has_double_quote;
			} else if bytes[i] == b'\''
				&& !has_double_quote
				&& bytes[idx_advance(&bytes[i..])] != b'"'
			{
				should_expand = !should_expand;
			}
			if i < bytes.len() - 1
				&& bytes[i] == b'$'
				&& should_expand
				&& !b"$()".contains(&bytes[i + 1])
			{
				let key_byte_slice = &bytes[(i + 1)..=(idx_advance(&bytes[i..]) + i)];
				// advance by key length in source string
				i += key_byte_slice.len();
				let expansion = if key_byte_slice.is_empty() {
					"$".to_string()
				} else if let Some(expansion) = self.get_slice(key_byte_slice) {
					expansion.to_string()
				} else {
					String::new()
				};
				ret.push_str(&expansion);
			} else {
				ret.push(bytes[i].into());
			}
			i += 1;
		}
		*input_expander = ret;
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
