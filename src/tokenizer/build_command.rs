use crate::prelude::*;

impl t_token {
	pub fn get_vec_cstr_token(token: &mut Self) -> Vec<CString> {
		let mut i = 0;
		if token.cmd_args_vec.is_empty() || token.cmd_args_vec[0].elem_str.is_empty() {
			panic!("token for get_cmd_arr shall not be null")
		}
		let mut vec_cstr = Vec::new();
		while i < token.cmd_args_vec.len() {
			if token.cmd_args_vec[i].type_0 != REDIR {
				// let cstr_ref = unsafe { CStr::from_ptr(token.cmd_args_vec[i].elem) };
				let cstring = CString::new(token.cmd_args_vec[i].elem_str.clone());
				vec_cstr.push(cstring.unwrap());
			}
			i += 1;
		}
		vec_cstr
	}
}
