use ::libc;
use libc::free;

use libft_rs::{
	ft_strchr::ft_strchr, ft_strdup::ft_strdup, ft_strlen::ft_strlen, ft_substr::ft_substr,
};
use libutils_rs::src::string::{append_char::append_char_str, join_strings::free_both_join};

use crate::size_t;

use super::get_env::get_env;

unsafe extern "C" fn check_index_advance(
	mut s: *const libc::c_char,
	mut i: libc::c_int,
) -> libc::c_int {
	let mut count: libc::c_int = 0;
	count = 0 as libc::c_int;
	while *s.offset((i + count) as isize) as libc::c_int != 0
		&& *s.offset((i + count + 1 as libc::c_int) as isize) as libc::c_int != 0
		&& (ft_strchr(
			b"$\"'/? )(\0" as *const u8 as *const libc::c_char,
			*s.offset((i + count + 1 as libc::c_int) as isize) as libc::c_int,
		))
		.is_null()
	{
		count += 1;
	}
	if *s.offset((i + count) as isize) as libc::c_int != 0
		&& *s.offset((i + count + 1 as libc::c_int) as isize) as libc::c_int == '?' as i32
	{
		count += 1 as libc::c_int;
	}
	count
}
unsafe extern "C" fn expand_inside(
	mut key: *mut libc::c_char,
	mut env: *const *mut libc::c_char,
	mut i: *mut libc::c_int,
) -> *mut libc::c_char {
	let len: size_t = ft_strlen(key);
	let mut ret: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	ret = std::ptr::null_mut::<libc::c_char>();
	if *key != 0 {
		ret = get_env(env, key);
	}
	if ret.is_null() {
		ret = ft_strdup(b"\0" as *const u8 as *const libc::c_char);
	}
	free(key as *mut libc::c_void);
	if ret.is_null() {
		return std::ptr::null_mut::<libc::c_char>();
	}
	*i = (*i as libc::c_ulong).wrapping_add(len) as libc::c_int as libc::c_int;
	ret
}
unsafe extern "C" fn check_quotes(
	mut s: *const libc::c_char,
	mut expand_0: *mut bool,
	mut double_quote: *mut libc::c_int,
) {
	if *s as libc::c_int == '"' as i32 && *double_quote == 0 as libc::c_int {
		*double_quote = 1 as libc::c_int;
	} else if *s as libc::c_int == '"' as i32 && *double_quote == 1 as libc::c_int {
		*double_quote = 0 as libc::c_int;
	}
	if *s as libc::c_int == '\'' as i32
		&& *double_quote == 0 as libc::c_int
		&& *s.offset(check_index_advance(s, 0 as libc::c_int) as isize) as libc::c_int != '"' as i32
	{
		*expand_0 = !*expand_0;
	}
}
unsafe extern "C" fn expand(
	mut s: *const libc::c_char,
	mut env: *const *mut libc::c_char,
) -> *mut libc::c_char {
	let mut i: libc::c_int = 0;
	let mut expand_0: bool = false;
	let mut double_quote: libc::c_int = 0;
	let mut ret: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut key: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	i = -(1 as libc::c_int);
	expand_0 = 1 as libc::c_int != 0;
	double_quote = 0 as libc::c_int;
	ret = ft_strdup(b"\0" as *const u8 as *const libc::c_char);
	loop {
		i += 1;
		if *s.offset(i as isize) == 0 {
			break;
		}
		check_quotes(&*s.offset(i as isize), &mut expand_0, &mut double_quote);
		if expand_0 as libc::c_int != 0
			&& (*s.offset(i as isize) as libc::c_int == '$' as i32
				&& *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int != 0
				&& (ft_strchr(
					b"$()\0" as *const u8 as *const libc::c_char,
					*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int,
				))
				.is_null())
		{
			key = ft_substr(
				s,
				(i + 1 as libc::c_int) as libc::c_uint,
				check_index_advance(s, i) as size_t,
			);
			if key.is_null() {
				free(ret as *mut libc::c_void);
				crate::utils::error::eprint(b"alloc fail!\0" as *const u8 as *const libc::c_char);
				return std::ptr::null_mut::<libc::c_void>() as *mut libc::c_char;
			}
			ret = free_both_join(ret, expand_inside(key, env, &mut i));
		} else {
			ret = append_char_str(ret, *s.offset(i as isize));
		}
	}
	ret
}
#[no_mangle]
pub unsafe extern "C" fn expander(
	mut input_expander: *const libc::c_char,
	mut env: *const *mut libc::c_char,
) -> *mut libc::c_char {
	if input_expander.is_null() || env.is_null() || (*env).is_null() || *input_expander == 0 {
		return std::ptr::null_mut::<libc::c_char>();
	}
	if (ft_strchr(input_expander, '$' as i32)).is_null() {
		return ft_strdup(input_expander);
	}
	expand(input_expander, env)
}
