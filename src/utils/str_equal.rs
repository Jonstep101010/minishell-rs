use ::libc;
extern "C" {
	fn ft_strlen(str: *const libc::c_char) -> size_t;
	fn ft_strnstr(
		big: *const libc::c_char,
		little: *const libc::c_char,
		len: size_t,
	) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
