use ::libc;
use libc::free;
use libft_rs::{ft_isascii::ft_isascii, ft_strdup::ft_strdup, ft_strtrim::ft_strtrim};
use libutils_rs::src::string::append_char::append_char_str;
unsafe extern "C" fn collect_as_ascii(mut readline_line: *mut libc::c_char) -> *mut libc::c_char {
	let mut i: libc::c_int = 0;
	let mut collected_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	i = 0 as libc::c_int;
	collected_line = std::ptr::null_mut::<libc::c_char>();
	if !readline_line.is_null() && *readline_line == 0 {
		free(readline_line as *mut libc::c_void);
		return ft_strdup(b"\0" as *const u8 as *const libc::c_char);
	}
	while !readline_line.is_null() && *readline_line.offset(i as isize) as libc::c_int != 0 {
		if ft_isascii(*readline_line.offset(i as isize) as libc::c_int) != 0 {
			collected_line = append_char_str(collected_line, *readline_line.offset(i as isize));
			if collected_line.is_null() {
				free(readline_line as *mut libc::c_void);
				return std::ptr::null_mut::<libc::c_void>() as *mut libc::c_char;
			}
		}
		i += 1;
	}
	if !readline_line.is_null() {
		free(readline_line as *mut libc::c_void);
	}
	collected_line
}
#[no_mangle]
pub unsafe extern "C" fn get_input(mut rl_prompt: *mut libc::c_char) -> *mut libc::c_char {
	static mut trim: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
	let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	if rl_prompt.is_null() {
		free(trim as *mut libc::c_void);
		return std::ptr::null_mut::<libc::c_void>() as *mut libc::c_char;
	}
	line = collect_as_ascii(rl_prompt);
	if line.is_null() {
		return std::ptr::null_mut::<libc::c_char>();
	}
	trim = ft_strtrim(
		line,
		b" \t\n\r\x0B\x0C\0" as *const u8 as *const libc::c_char,
	);
	free(line as *mut libc::c_void);
	if trim.is_null() {
		return std::ptr::null_mut::<libc::c_char>();
	}
	trim
}
