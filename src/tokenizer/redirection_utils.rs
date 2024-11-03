use ::libc;
extern "C" {
	fn free_null(p: *mut libc::c_void);
	fn memsize(arr: *mut libc::c_void, size: size_t) -> size_t;
	fn free(_: *mut libc::c_void);
	fn ft_strncmp(s1: *const libc::c_char, s2: *const libc::c_char, n: size_t) -> libc::c_int;
	fn ft_strdup(s: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
use crate::t_arg;
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
#[no_mangle]
pub unsafe extern "C" fn rm_prefix_redir_word(mut arg: *mut t_arg) {
	let mut i: size_t = 0;
	let mut len: size_t = 0;
	if arg.is_null() {
		return;
	}
	i = 0 as libc::c_int as size_t;
	len = memsize(
		arg as *mut libc::c_void,
		::core::mem::size_of::<t_arg>() as libc::c_ulong,
	);
	while !((*arg.offset(i as isize)).elem).is_null() {
		if (*arg.offset(i as isize)).type_0 as libc::c_uint
			== REDIR_REMOVED as libc::c_int as libc::c_uint
			&& !((*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)).elem)
				.is_null()
		{
			free((*arg.offset(i as isize)).elem as *mut libc::c_void);
			(*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)).type_0 =
				REDIR;
			(*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)).redir =
				(*arg.offset(i as isize)).redir;
			while i < len {
				*arg.offset(i as isize) =
					*arg.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
				i = i.wrapping_add(1);
				i;
			}
			return rm_prefix_redir_word(arg);
		}
		i = i.wrapping_add(1);
		i;
	}
}
#[no_mangle]
pub unsafe extern "C" fn parse_redir_types(mut arg: *mut t_arg) {
	let mut i: libc::c_int = 0;
	let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
	i = -(1 as libc::c_int);
	loop {
		i += 1;
		if ((*arg.offset(i as isize)).elem).is_null() {
			break;
		}
		if (*arg.offset(i as isize)).type_0 as libc::c_uint == REDIR as libc::c_int as libc::c_uint
		{
			if (*arg.offset(i as isize)).redir as libc::c_uint
				== INPUT_REDIR as libc::c_int as libc::c_uint
				|| (*arg.offset(i as isize)).redir as libc::c_uint
					== OUTPUT_REDIR as libc::c_int as libc::c_uint
			{
				tmp = ft_strdup(
					&mut *((*arg.offset(i as isize)).elem).offset(1 as libc::c_int as isize),
				);
			} else {
				tmp = ft_strdup(
					&mut *((*arg.offset(i as isize)).elem).offset(2 as libc::c_int as isize),
				);
			}
			free_null(
				&mut (*arg.offset(i as isize)).elem as *mut *mut libc::c_char as *mut libc::c_void,
			);
			let ref mut fresh0 = (*arg.offset(i as isize)).elem;
			*fresh0 = tmp;
		}
	}
}
unsafe extern "C" fn set_type_redir(mut cmd_arg: *mut t_arg) {
	if (*cmd_arg).redir as libc::c_uint == APPEND as libc::c_int as libc::c_uint
		|| (*cmd_arg).redir as libc::c_uint == HEREDOC as libc::c_int as libc::c_uint
	{
		if *((*cmd_arg).elem).offset(2 as libc::c_int as isize) == 0 {
			(*cmd_arg).type_0 = REDIR_REMOVED;
		} else {
			(*cmd_arg).type_0 = REDIR;
		}
	} else if (*cmd_arg).redir as libc::c_uint == OUTPUT_REDIR as libc::c_int as libc::c_uint
		|| (*cmd_arg).redir as libc::c_uint == INPUT_REDIR as libc::c_int as libc::c_uint
	{
		if *((*cmd_arg).elem).offset(1 as libc::c_int as isize) == 0 {
			(*cmd_arg).type_0 = REDIR_REMOVED;
		} else {
			(*cmd_arg).type_0 = REDIR;
		}
	}
}
#[no_mangle]
pub unsafe extern "C" fn check_redirections(mut cmd_args: *mut t_arg) -> e_redir {
	let mut ii: size_t = 0;
	let mut redir: bool = false;
	ii = 0 as libc::c_int as size_t;
	redir = 0 as libc::c_int != 0;
	while !((*cmd_args.offset(ii as isize)).elem).is_null() {
		if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b">>\0" as *const u8 as *const libc::c_char,
			2 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = APPEND;
		} else if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b">\0" as *const u8 as *const libc::c_char,
			1 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = OUTPUT_REDIR;
		} else if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b"<<\0" as *const u8 as *const libc::c_char,
			2 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = HEREDOC;
		} else if ft_strncmp(
			(*cmd_args.offset(ii as isize)).elem,
			b"<\0" as *const u8 as *const libc::c_char,
			1 as libc::c_int as size_t,
		) == 0 as libc::c_int
		{
			(*cmd_args.offset(ii as isize)).redir = INPUT_REDIR;
		}
		if (*cmd_args.offset(ii as isize)).redir as libc::c_uint
			!= NO_REDIR as libc::c_int as libc::c_uint
		{
			set_type_redir(&mut *cmd_args.offset(ii as isize));
			redir = 1 as libc::c_int != 0;
		}
		ii = ii.wrapping_add(1);
		ii;
	}
	return redir as e_redir;
}
