use ::libc;
use libft_rs::ft_calloc::ft_calloc;

use crate::size_t;

pub unsafe fn bool_arr_zeroing(mut len: size_t) -> *mut bool {
	let mut i: usize = 0;
	let mut ignore: *mut bool = ft_calloc(
		len.wrapping_add(1 as libc::c_ulong),
		::core::mem::size_of::<bool>() as libc::c_ulong,
	) as *mut bool;
	if ignore.is_null() {
		return std::ptr::null_mut::<bool>();
	}
	while i < len.wrapping_add(1) as usize {
		*ignore.add(i) = false;
		i = i.wrapping_add(1);
	}
	ignore
}

pub unsafe fn range_ignore(
	mut s: *const libc::c_char,
	mut ignore: *mut bool,
	mut c: libc::c_uchar,
) {
	let mut i = 0;
	while *s.offset(i) != 0 {
		if *s.offset(i) as u8 == c && *ignore.offset(i) as u8 == 0 {
			*ignore.offset(i) = 1 != 0;
			i += 1;
			while *s.offset(i) != 0 && *s.offset(i) as u8 != c {
				*ignore.offset(i) = 1 != 0;
				i += 1;
			}
			if *s.offset(i) as u8 == c {
				*ignore.offset(i) = 1 != 0;
			}
			while *s.offset(i) as u8 != c {
				*ignore.offset(i) = false;
				i -= 1;
			}
		} else {
			i += 1;
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::utils::bool_array::range_ignore;
	use crate::utils::bool_array::{bool_arr_zeroing, libc::strlen};

	use libc::size_t;

	unsafe fn support_bool_arr_zeroing(mut ignore: *mut *mut bool, mut len: usize) {
		*ignore = super::bool_arr_zeroing(len as u64) as *mut bool;
	}

	unsafe fn support_expected(mut s: *const libc::c_char) -> *mut bool {
		let mut arr: *mut bool = std::ptr::null_mut::<bool>();
		let mut len = strlen(s);
		support_bool_arr_zeroing(&mut arr, len as size_t);
		if arr.is_null() {
			return std::ptr::null_mut::<bool>();
		}
		let mut i = 0;
		while i <= len {
			if *s.add(i) as u8 == b'1' {
				*arr.add(i) = true;
			} else {
				*arr.add(i) = false;
			}
			i += 1;
		}
		return arr;
	}
	unsafe fn support_ranges_test0(mut arr: *const bool) {
		assert_eq!(false, *arr.add(25));
		assert_eq!(false, *arr.add(29));
		assert_eq!(false, *arr.add(36));
		assert_eq!(false, *arr.add(41));
		assert_eq!(false, *arr.add(45));
		assert_eq!(false, *arr.add(52));
		assert_eq!(false, *arr.add(61));
		let mut i = 0;
		while i < 17 {
			assert_eq!(false, *arr.add(i));
			i += 1;
		}
		let mut i_0 = 17;
		while i_0 <= 24 {
			assert_eq!(true, *arr.add(i_0));
			i_0 += 1;
		}
		let mut i_1 = 24 + 2;
		while i_1 <= 24 + 4 {
			assert_eq!(true, *arr.add(i_1));
			i_1 += 1;
		}
		let mut i_2 = 30;
		while i_2 <= 35 {
			assert_eq!(true, *arr.add(i_2));
			i_2 += 1;
		}
		let mut i_3 = 37;
		while i_3 <= 40 {
			assert_eq!(true, *arr.add(i_3));
			i_3 += 1;
		}
		let mut i_4 = 42;
		while i_4 <= 44 {
			assert_eq!(true, *arr.add(i_4));
			i_4 += 1;
		}
		let mut i_5 = 46;
		while i_5 <= 51 {
			assert_eq!(true, *arr.add(i_5));
			i_5 += 1;
		}
		let mut i_6 = 53;
		while i_6 <= 60 {
			assert_eq!(true, *arr.add(i_6));
			i_6 += 1;
		}
	}
	#[test]
	fn test_ignore_can_work0() {
		unsafe {
			let mut s: [libc::c_char; 62] =
				*::core::mem::transmute::<&[u8; 62], &mut [libc::c_char; 62]>(
					b"this is my input \"ignore\" 't' 'this' \"is\" 'a' \"test\" 'string'\0",
				);
			let _expected: *mut bool = support_expected(
				b"00000000000000000111111110111011111101111011101111110111111110\0" as *const u8
					as *const libc::c_char,
			);
			let mut len = libc::strlen(s.as_mut_ptr());
			let mut arr = bool_arr_zeroing(len as u64);
			if arr.is_null() {
				assert!(false);
			}
			range_ignore(s.as_mut_ptr(), arr, b'"');
			range_ignore(s.as_mut_ptr(), arr, b'\'');
			let mut i = 0;
			while i < libc::strlen(s.as_mut_ptr()) {
				i += 1;
			}
			support_ranges_test0(arr);
			libc::free(arr as *mut libc::c_void);
		}
	}
	#[test]
	fn test_range_ignore_sq_0() {
		unsafe {
			let mut arr: *mut bool = 0 as *mut bool;
			let mut s: [libc::c_char; 26] = *::core::mem::transmute::<
				&[u8; 26],
				&mut [libc::c_char; 26],
			>(b"this is my input 'ignore'\0");
			support_bool_arr_zeroing(&mut arr, libc::strlen(s.as_mut_ptr()));
			let mut i = 0;
			while i <= libc::strlen(s.as_mut_ptr()) {
				i += 1;
			}
			super::range_ignore(s.as_mut_ptr(), arr, b'\'');
			let mut i_0 = 0;
			while i_0 <= 16 {
				i_0 += 1;
			}
			let mut i_1 = 17;
			while i_1 <= 24 {
				i_1 += 1;
			}
			libc::free(arr as *mut libc::c_void);
		}
	}
	#[test]
	fn test_range_ignore_dq_0() {
		unsafe {
			let mut arr: *mut bool = 0 as *mut bool;
			let mut s: [libc::c_char; 26] = *::core::mem::transmute::<
				&[u8; 26],
				&mut [libc::c_char; 26],
			>(b"this is my input \"ignore\"\0");
			support_bool_arr_zeroing(&mut arr, libc::strlen(s.as_mut_ptr()) as usize);
			let mut i = 0;
			while i <= libc::strlen(s.as_mut_ptr()) {
				i += 1;
			}
			super::range_ignore(s.as_mut_ptr(), arr, b'"');
			let mut i_0 = 0;
			while i_0 < 17 {
				i_0 += 1;
			}
			let mut i_1 = 17;
			while i_1 <= 24 {
				i_1 += 1;
			}
			libc::free(arr as *mut libc::c_void);
		}
	}
}
