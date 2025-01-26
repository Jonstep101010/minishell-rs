use std::ffi::CStr;

use ::libc;
use libc::free;
use libft_rs::{
	ft_calloc::ft_calloc, ft_memcpy::ft_memcpy, ft_strlen::ft_strlen, ft_strncmp::ft_strncmp,
};
use libutils_rs::src::{
	array::{arr_free::arr_free, arr_len::arr_len},
	string::str_cchr::str_cchr,
	utils::free_mem::free_null,
};

use crate::{
	builtins::{
		cd::builtin_cd, echo::echo, env::builtin_env, exit::builtin_exit, export::builtin_export,
		pwd::builtin_pwd, unset::builtin_unset,
	},
	environment::{Env, expander::expander},
	execution::exec_bin::exec_bin,
	parser::{interpret_quotes::do_quote_bs, split_outside_quotes::split_outside_quotes},
	prelude::*,
};

#[allow(unused_mut)]
unsafe fn init_token(mut size: size_t) -> *mut t_token {
	let template: t_token = {
		t_token {
			// cmd_args: std::ptr::null_mut::<t_arg>(),
			has_redir: false,
			tmp_arr: std::ptr::null_mut::<*mut libc::c_char>(),
			bin: std::ffi::CString::new("").unwrap(),
			cmd_func: None,
			split_non_quoted: String::new(),
			cmd_args_vec: vec![],
		}
	};
	let mut token: *mut t_token =
		ft_calloc(size + 1, ::core::mem::size_of::<t_token>() as libc::c_ulong) as *mut t_token;
	while !token.is_null() && {
		let fresh1 = size;
		size = size.wrapping_sub(1);
		fresh1 != 0
	} {
		ft_memcpy(
			&mut *token.add(size as usize) as *mut t_token as *mut libc::c_void,
			&template as *const t_token as *const libc::c_void,
			::core::mem::size_of::<t_token>() as libc::c_ulong,
		);
	}
	std::mem::forget(template);
	token
}

unsafe fn get_tokens(mut trimmed_line: *const libc::c_char) -> Option<(*mut t_token, usize)> {
	let mut split_pipes: *mut *mut libc::c_char = split_outside_quotes(trimmed_line, c"|".as_ptr());
	if split_pipes.is_null() {
		panic!("alloc fail token");
	}
	if (*split_pipes).is_null() {
		arr_free(split_pipes);
		return None;
	}
	let mut token: *mut t_token = init_token(arr_len(split_pipes));
	if token.is_null() {
		panic!("alloc fail token");
	}
	let mut i = 0;
	while !(*split_pipes.add(i)).is_null() {
		(*token.add(i)).tmp_arr =
			split_outside_quotes(*split_pipes.add(i), c" \t\n\r\x0B\x0C".as_ptr());
		if ((*token).tmp_arr).is_null() {
			// @audit will leak!
			return None;
		}
		i += 1;
	}
	free(split_pipes as *mut libc::c_void);
	Some((token, i))
}

unsafe fn setup_token(mut token: *mut t_token, env: &Env) -> Option<()> {
	if token.is_null() || ((*token).tmp_arr).is_null() {
		return None;
	}
	(*token).cmd_args_vec = vec![
		t_arg {
			elem: std::ptr::null_mut::<libc::c_char>(),
			type_0: STRING,
			redir: None,
		};
		arr_len((*token).tmp_arr) as usize
	];
	if ((*token).cmd_args_vec).is_empty() {
		arr_free((*token).tmp_arr);
		return None;
	}
	let mut ii = 0;
	while !(*((*token).tmp_arr).add(ii)).is_null() {
		(*token).cmd_args_vec[ii].elem = *((*token).tmp_arr).add(ii);
		if (*token).cmd_func != Some(builtin_env as unsafe fn(&mut t_shell, *mut t_token) -> i32) {
			let mut token_cmd_args_elem = (*token).cmd_args_vec[ii].elem;
			// expand if allowed
			if str_cchr(token_cmd_args_elem, '$' as i32 as libc::c_char) != 0 {
				// we know this is non-null
				let c_str = CStr::from_ptr(token_cmd_args_elem);
				let mut tmp = expander(c_str, env)?;
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
	free_null(&mut (*token).tmp_arr as *mut *mut *mut libc::c_char as *mut libc::c_void);
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
	let trimmed_line = std::ffi::CString::new(trimmed_line).unwrap();
	let tuple = get_tokens(trimmed_line.as_ptr())?;
	shell.token_len = Some(tuple.1);
	shell.token = tuple.0;
	let mut i = 0;
	while i < shell.token_len.unwrap() {
		setup_token(&mut *(shell.token).add(i), &shell.env)?;
		// if (*(shell.token).add(i)).cmd_args_vec.is_empty() {
		// 	return None;
		// }
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
		// (*token).cmd_func = match CStr::from_ptr((*((*token).cmd_args).add(i)).elem).to_bytes() {
		(*(shell.token).add(i)).cmd_func =
			match CStr::from_ptr((*(shell.token).add(i)).cmd_args_vec[ii].elem).to_bytes() {
				b"echo" => Some(echo as unsafe fn(&mut t_shell, *mut t_token) -> i32),
				b"cd" => Some(builtin_cd as unsafe fn(&mut t_shell, *mut t_token) -> i32),
				b"pwd" => Some(builtin_pwd as unsafe fn(&mut t_shell, *mut t_token) -> i32),
				b"export" => Some(builtin_export as unsafe fn(&mut t_shell, *mut t_token) -> i32),
				b"unset" => Some(builtin_unset as unsafe fn(&mut t_shell, *mut t_token) -> i32),
				b"env" => Some(builtin_env as unsafe fn(&mut t_shell, *mut t_token) -> i32),
				b"exit" => Some(builtin_exit as unsafe fn(&mut t_shell, *mut t_token) -> i32),
				_ => Some(exec_bin as unsafe fn(&mut t_shell, *mut t_token) -> i32),
			};
		// rm_quotes
		let mut quote = 0;
		let mut iii = 0;
		while iii < (*(shell.token).add(i)).cmd_args_vec.len()
			&& !(*(shell.token).add(i)).cmd_args_vec[iii].elem.is_null()
		{
			let mut tmp: *mut libc::c_char =
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
		let mut split_pipes =
			crate::parser::split_outside_quotes::split_non_quoted(trimmed_line, "|");
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
