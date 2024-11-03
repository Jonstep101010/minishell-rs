use ::libc;
extern "C" {
	fn ft_calloc(nitems: size_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn bool_arr_zeroing(mut len: size_t) -> *mut bool {
	let mut i: size_t = 0;
	let mut ignore: *mut bool = std::ptr::null_mut::<bool>();
	i = 0 as libc::c_int as size_t;
	ignore = ft_calloc(
		len.wrapping_add(1 as libc::c_int as libc::c_ulong),
		::core::mem::size_of::<bool>() as libc::c_ulong,
	) as *mut bool;
	if ignore.is_null() {
		return std::ptr::null_mut::<bool>();
	}
	while i < len.wrapping_add(1 as libc::c_int as libc::c_ulong) {
		*ignore.offset(i as isize) = 0 as libc::c_int != 0;
		i = i.wrapping_add(1);
	}
	ignore
}
#[no_mangle]
pub unsafe extern "C" fn range_ignore(
	mut s: *const libc::c_char,
	mut ignore: *mut bool,
	mut c: libc::c_uchar,
) {
	let mut i: libc::c_int = 0;
	i = 0 as libc::c_int;
	while *s.offset(i as isize) != 0 {
		if *s.offset(i as isize) as libc::c_int == c as libc::c_int
			&& *ignore.offset(i as isize) as libc::c_int == 0 as libc::c_int
		{
			*ignore.offset(i as isize) = 1 as libc::c_int != 0;
			i += 1;
			while *s.offset(i as isize) as libc::c_int != 0
				&& *s.offset(i as isize) as libc::c_int != c as libc::c_int
			{
				*ignore.offset(i as isize) = 1 as libc::c_int != 0;
				i += 1;
			}
			if *s.offset(i as isize) as libc::c_int == c as libc::c_int {
				*ignore.offset(i as isize) = 1 as libc::c_int != 0;
			}
			while *s.offset(i as isize) as libc::c_int != c as libc::c_int {
				*ignore.offset(i as isize) = 0 as libc::c_int != 0;
				i -= 1;
			}
		} else {
			i += 1;
		}
	}
}
