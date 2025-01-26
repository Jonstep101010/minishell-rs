use std::ffi::{CStr, CString};

use ::libc;
use libc::free;
use libft_rs::{
	ft_calloc::ft_calloc, ft_memcpy::ft_memcpy, ft_strlen::ft_strlen, ft_strncmp::ft_strncmp,
};
use libutils_rs::src::{
	array::{arr_free::arr_free, arr_len::arr_len},
	string::str_cchr::str_cchr,
};

use crate::{
	builtins::{
		cd::builtin_cd, echo::echo, env::builtin_env, exit::builtin_exit, export::builtin_export,
		pwd::builtin_pwd, unset::builtin_unset,
	},
	environment::{Env, expander::expander},
	execution::exec_bin::exec_bin,
	parser::{
		interpret_quotes::do_quote_bs,
		split_outside_quotes::{split_non_quoted, split_outside_quotes},
	},
	prelude::*,
};

#[allow(unused_mut)]
unsafe fn init_token(mut size: usize) -> *mut t_token {
	let template: t_token = {
		t_token {
			has_redir: false,
			cmd_func: None,
			split_non_quoted: String::new(),
			cmd_args_vec: vec![],
		}
	};
	let mut token: *mut t_token = ft_calloc(
		size as u64 + 1,
		::core::mem::size_of::<t_token>() as libc::c_ulong,
	) as *mut t_token;
	while !token.is_null() && {
		let fresh1 = size;
		size = size.wrapping_sub(1);
		fresh1 != 0
	} {
		ft_memcpy(
			&mut *token.add(size) as *mut t_token as *mut libc::c_void,
			&template as *const t_token as *const libc::c_void,
			::core::mem::size_of::<t_token>() as libc::c_ulong,
		);
	}
	std::mem::forget(template);
	token
}

unsafe fn setup_token(token: *mut t_token, token_split: *mut *mut c_char, env: &Env) -> Option<()> {
	(*token).cmd_args_vec = vec![
		t_arg {
			elem: std::ptr::null_mut::<libc::c_char>(),
			type_0: STRING,
			redir: None,
		};
		arr_len(token_split) as usize
	];
	if ((*token).cmd_args_vec).is_empty() {
		arr_free(token_split);
		return None;
	}
	let mut ii = 0;
	while !(*(token_split).add(ii)).is_null() {
		(*token).cmd_args_vec[ii].elem = *(token_split).add(ii);
		if (*token).cmd_func
			!= Some(builtin_env as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32)
		{
			let mut token_cmd_args_elem = (*token).cmd_args_vec[ii].elem;
			// expand if allowed
			if str_cchr(token_cmd_args_elem, '$' as i32 as libc::c_char) != 0 {
				// we know this is non-null
				let c_str = CStr::from_ptr(token_cmd_args_elem);
				let tmp = expander(c_str, env)?;
				if ft_strncmp(
					tmp.as_ptr(),
					token_cmd_args_elem,
					((if ft_strlen(tmp.as_ptr()) > ft_strlen(token_cmd_args_elem) {
						ft_strlen(tmp.as_ptr())
					} else {
						ft_strlen(token_cmd_args_elem)
					}) == 0 as libc::c_ulong) as i32 as size_t,
				) == 0
				{
					free(token_cmd_args_elem as *mut libc::c_void);
					let fresh0 = &mut token_cmd_args_elem;
					*fresh0 = tmp.into_raw();
				}
			}
		}
		ii += 1;
	}
	Some(())
}

// fn rm_quotes(mut cmd_arg: Vec<t_arg>) {
// 	let mut quote = 0;
// 	let mut i = 0;
// 	loop {
// 		if i == cmd_arg.len() {
// 			break;
// 		}
// 		cmd_arg[i].elem = rs_do_quote_bs((*cmd_arg.add(i)).elem, &mut quote);
// 		i += 1;
// 	}
// }

pub unsafe fn tokenize(shell: &mut t_shell, trimmed_line: &str) -> Option<()> {
	let split_pipes = split_non_quoted(trimmed_line, "|");
	assert!(!split_pipes.is_empty());
	if split_pipes.first().unwrap().is_empty() {
		return None;
	}
	shell.token_len = Some(split_pipes.len());
	shell.token = init_token(shell.token_len.unwrap());
	if shell.token.is_null() {
		panic!("alloc fail token");
	}
	let mut i = 0;
	while i < shell.token_len.unwrap() {
		debug_assert!(!(shell.token).add(i).is_null());
		setup_token(
			&mut *(shell.token).add(i),
			split_outside_quotes(
				CString::new(split_pipes[i].clone()).unwrap().as_ptr(),
				c" \t\n\r\x0B\x0C".as_ptr(),
			),
			&shell.env,
		)?;
		super::redirection_utils::process_redirections((shell.token).add(i));
		let mut ii = 0;
		// check if the elem is null for a specific tokens' cmd_args
		while ii < (*(shell.token).add(i)).cmd_args_vec.len()
			&& !(*(shell.token).add(i)).cmd_args_vec[ii].elem.is_null()
		{
			if (*(shell.token).add(i)).cmd_args_vec[ii].type_0 != REDIR {
				break;
			}
			ii += 1;
		}
		// set_cmd_func
		(*(shell.token).add(i)).cmd_func =
			match CStr::from_ptr((*(shell.token).add(i)).cmd_args_vec[ii].elem).to_bytes() {
				b"echo" => Some(echo as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32),
				b"cd" => Some(builtin_cd as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32),
				b"pwd" => {
					Some(builtin_pwd as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32)
				}
				b"export" => {
					Some(builtin_export as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32)
				}
				b"unset" => {
					Some(builtin_unset as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32)
				}
				b"env" => {
					Some(builtin_env as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32)
				}
				b"exit" => {
					Some(builtin_exit as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32)
				}
				_ => Some(exec_bin as unsafe fn(&mut Env, Option<*mut *const c_char>) -> i32),
			};
		// rm_quotes
		let mut quote = 0;
		let mut iii = 0;
		while iii < (*(shell.token).add(i)).cmd_args_vec.len()
			&& !(*(shell.token).add(i)).cmd_args_vec[iii].elem.is_null()
		{
			let tmp: *mut libc::c_char =
				do_quote_bs((*(shell.token).add(i)).cmd_args_vec[iii].elem, &mut quote)
					as *mut libc::c_char;
			if tmp.is_null() {
				break;
			}
			(*(shell.token).add(i)).cmd_args_vec[iii].elem = tmp;
			iii += 1;
		}
		i += 1;
	}
	Some(())
}

// @todo testing before usage
impl t_shell {
	///
	/// future replacement for `get_tokens`
	pub fn create_tokens(&mut self, trimmed_line: &str) {
		let split_pipes = crate::parser::split_outside_quotes::split_non_quoted(trimmed_line, "|");
		self.token_vec = split_pipes
			.iter()
			.map(|single_pipe| t_token::new(single_pipe.to_owned()))
			.collect();
		self.token_len = if self.token_vec.is_empty() {
			None
		} else {
			Some(self.token_vec.len())
		}
	}
}
