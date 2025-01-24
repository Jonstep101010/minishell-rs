use super::token_utils::init_token;
use crate::parser::split_outside_quotes::split_non_quoted;
use crate::t_shell;
use crate::{parser::split_outside_quotes::split_outside_quotes, t_token};
use ::libc;
use libc::free;
use libft_rs::ft_strdup::ft_strdup;
use libutils_rs::src::array::append_str::append_str_arr_free;
use libutils_rs::src::array::arr_free::arr_free;
use libutils_rs::src::array::arr_len::arr_len;

#[unsafe(no_mangle)]
pub unsafe fn get_cmd_arr_token(mut token: *mut t_token) -> *mut *mut libc::c_char {
	let mut i = 0;
	let mut cmd_arr: *mut *mut libc::c_char = std::ptr::null_mut::<*mut libc::c_char>();
	if token.is_null() || ((*token).cmd_args).is_null() {
		return std::ptr::null_mut::<*mut libc::c_char>();
	}
	if !((*((*token).cmd_args).add(0)).elem).is_null() {
		while !((*((*token).cmd_args).add(i)).elem).is_null() {
			if (*((*token).cmd_args).add(i)).type_0 as libc::c_uint
				!= crate::e_arg::REDIR as i32 as libc::c_uint
			{
				cmd_arr =
					append_str_arr_free(cmd_arr, ft_strdup((*((*token).cmd_args).add(i)).elem));
				if cmd_arr.is_null() {
					return std::ptr::null_mut::<*mut libc::c_char>();
				}
			}
			i += 1;
		}
	}
	cmd_arr
}
#[unsafe(no_mangle)]
pub unsafe fn get_tokens(mut trimmed_line: *const libc::c_char) -> Option<(*mut t_token, usize)> {
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
		(*token.add(i)).split_pipes = *split_pipes.add(i);
		i += 1;
	}
	free(split_pipes as *mut libc::c_void);
	Some((token, i))
}

impl t_shell {
	///
	/// future replacement for `get_tokens`
	pub fn create_tokens(&mut self, trimmed_line: &str) {
		let mut split_pipes = split_non_quoted(trimmed_line, "|");
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
