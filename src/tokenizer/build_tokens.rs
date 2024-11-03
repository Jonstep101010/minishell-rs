use ::libc;

use crate::t_shell;
extern "C" {
	fn arr_len(arr: *const *mut libc::c_char) -> size_t;
	fn arr_free(arr: *mut *mut libc::c_char);
	fn set_cmd_func(cmd: *const libc::c_char, token: *mut t_token);
	fn get_tokens(trimmed_line: *const libc::c_char) -> *mut t_token;
	fn destroy_all_tokens(shell: *mut t_shell);
	fn init_cmdargs(size: size_t) -> *mut t_arg;
	fn rm_prefix_redir_word(arg: *mut t_arg);
	fn parse_redir_types(arg: *mut t_arg);
	fn check_redirections(cmd_args: *mut t_arg) -> e_redir;
	fn builtin_env(shell: *mut t_shell, token: *mut t_token) -> libc::c_int;
	fn free(_: *mut libc::c_void);
	fn ft_strlen(str: *const libc::c_char) -> size_t;
	fn ft_strncmp(s1: *const libc::c_char, s2: *const libc::c_char, n: size_t) -> libc::c_int;
	fn str_cchr(s: *const libc::c_char, c: libc::c_char) -> libc::c_int;
	fn free_null(p: *mut libc::c_void);
	fn expander(
		input_expander: *const libc::c_char,
		env: *const *mut libc::c_char,
	) -> *mut libc::c_char;
	fn split_outside_quotes(
		to_split: *const libc::c_char,
		set: *const libc::c_char,
	) -> *mut *mut libc::c_char;
	fn do_quote_bs(s: *const libc::c_char, quote: *mut libc::c_int) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
	pub c_iflag: tcflag_t,
	pub c_oflag: tcflag_t,
	pub c_cflag: tcflag_t,
	pub c_lflag: tcflag_t,
	pub c_line: cc_t,
	pub c_cc: [cc_t; 32],
	pub c_ispeed: speed_t,
	pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
use crate::t_arg;
use crate::t_token;
pub type e_redir = libc::c_uint;
pub const HEREDOC: e_redir = 4;
pub const APPEND: e_redir = 3;
pub const OUTPUT_REDIR: e_redir = 2;
pub const INPUT_REDIR: e_redir = 1;
pub const NO_REDIR: e_redir = 0;
pub type e_arg = libc::c_uint;
pub const REDIR_REMOVED: e_arg = 2;
pub const REDIR: e_arg = 1;
pub const STRING: e_arg = 0;
unsafe extern "C" fn expand_if_allowed(
	mut token: *mut t_token,
	mut ii: size_t,
	mut env: *const *mut libc::c_char,
) -> *mut libc::c_void {
	let mut tmp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	if (*token).cmd_func
		!= Some(builtin_env as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int)
		&& str_cchr(
			(*((*token).cmd_args).offset(ii as isize)).elem,
			'$' as i32 as libc::c_char,
		) != 0 as libc::c_int
	{
		tmp = expander((*((*token).cmd_args).offset(ii as isize)).elem, env);
		if tmp.is_null() {
			return std::ptr::null_mut::<libc::c_void>();
		}
		if ft_strncmp(
			tmp,
			(*((*token).cmd_args).offset(ii as isize)).elem,
			((if ft_strlen(tmp) > ft_strlen((*((*token).cmd_args).offset(ii as isize)).elem) {
				ft_strlen(tmp)
			} else {
				ft_strlen((*((*token).cmd_args).offset(ii as isize)).elem)
			}) == 0 as libc::c_int as libc::c_ulong) as libc::c_int as size_t,
		) != 0
		{
			free(tmp as *mut libc::c_void);
		} else {
			free((*((*token).cmd_args).offset(ii as isize)).elem as *mut libc::c_void);
			let fresh0 = &mut (*((*token).cmd_args).offset(ii as isize)).elem;
			*fresh0 = tmp;
		}
	}
	token as *mut libc::c_void
}
unsafe extern "C" fn setup_token(
	mut token: *mut t_token,
	mut env: *const *mut libc::c_char,
) -> *mut libc::c_void {
	let mut ii: size_t = 0;
	if token.is_null() || ((*token).split_pipes).is_null() {
		return std::ptr::null_mut::<libc::c_void>();
	}
	(*token).tmp_arr = split_outside_quotes(
		(*token).split_pipes,
		b" \t\n\r\x0B\x0C\0" as *const u8 as *const libc::c_char,
	);
	free_null(&mut (*token).split_pipes as *mut *mut libc::c_char as *mut libc::c_void);
	if ((*token).tmp_arr).is_null() {
		return std::ptr::null_mut::<libc::c_void>();
	}
	(*token).cmd_args = init_cmdargs(arr_len((*token).tmp_arr));
	if ((*token).cmd_args).is_null() {
		arr_free((*token).tmp_arr);
		return std::ptr::null_mut::<libc::c_void>();
	}
	ii = 0 as libc::c_int as size_t;
	while !(*((*token).tmp_arr).offset(ii as isize)).is_null() {
		let fresh1 = &mut (*((*token).cmd_args).offset(ii as isize)).elem;
		*fresh1 = *((*token).tmp_arr).offset(ii as isize);
		if (expand_if_allowed(token, ii, env)).is_null() {
			return std::ptr::null_mut::<libc::c_void>();
		}
		ii = ii.wrapping_add(1);
	}
	free_null(&mut (*token).tmp_arr as *mut *mut *mut libc::c_char as *mut libc::c_void);
	token as *mut libc::c_void
}
unsafe extern "C" fn rm_quotes(mut cmd_arg: *mut t_arg) {
	let mut tmp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut quote: libc::c_int = 0;
	let mut i: libc::c_int = 0;
	quote = 0 as libc::c_int;
	i = -(1 as libc::c_int);
	loop {
		i += 1;
		if ((*cmd_arg.offset(i as isize)).elem).is_null() {
			break;
		}
		tmp = do_quote_bs((*cmd_arg.offset(i as isize)).elem, &mut quote) as *mut libc::c_char;
		if tmp.is_null() {
			return;
		}
		free_null(
			&mut (*cmd_arg.offset(i as isize)).elem as *mut *mut libc::c_char as *mut libc::c_void,
		);
		let fresh2 = &mut (*cmd_arg.offset(i as isize)).elem;
		*fresh2 = tmp;
	}
}
unsafe extern "C" fn inner_loop(mut token: *mut t_token) -> *mut libc::c_void {
	let mut i: libc::c_int = 0;
	if check_redirections((*token).cmd_args) as u64 != 0 {
		(*token).has_redir = 1 as libc::c_int != 0;
		parse_redir_types((*token).cmd_args);
		rm_prefix_redir_word((*token).cmd_args);
	}
	i = 0 as libc::c_int;
	while !((*((*token).cmd_args).offset(i as isize)).elem).is_null() {
		if (*((*token).cmd_args).offset(i as isize)).type_0 as libc::c_uint
			!= REDIR as libc::c_int as libc::c_uint
		{
			break;
		}
		i += 1;
	}
	set_cmd_func((*((*token).cmd_args).offset(i as isize)).elem, token);
	rm_quotes((*token).cmd_args);
	token as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn tokenize(
	mut shell: *mut t_shell,
	mut trimmed_line: *const libc::c_char,
) -> *mut libc::c_void {
	let mut i: size_t = 0;
	i = 0 as libc::c_int as size_t;
	(*shell).token_len = 0 as libc::c_int as size_t;
	(*shell).token = get_tokens(trimmed_line);
	if ((*shell).token).is_null() {
		return std::ptr::null_mut::<libc::c_void>();
	}
	while !((*((*shell).token).offset((*shell).token_len as isize)).split_pipes).is_null() {
		(*shell).token_len = ((*shell).token_len).wrapping_add(1);
	}
	while i < (*shell).token_len {
		if (setup_token(&mut *((*shell).token).offset(i as isize), (*shell).env)).is_null() {
			destroy_all_tokens(shell);
			return std::ptr::null_mut::<libc::c_void>();
		}
		inner_loop(&mut *((*shell).token).offset(i as isize));
		i = i.wrapping_add(1);
	}
	(*shell).token as *mut libc::c_void
}
