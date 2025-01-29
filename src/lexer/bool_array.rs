pub trait BoolArray {
	fn zeroing(len: usize) -> Self;
	fn range_ignore(s: &[u8], ignore: &mut Self, c: u8);
}

impl BoolArray for Vec<bool> {
	fn zeroing(len: usize) -> Self {
		let mut vec = Vec::with_capacity(len + 1);
		vec.resize(len + 1, false);
		vec
	}
	fn range_ignore(s: &[u8], ignore: &mut Self, c: u8) {
		let mut i = 0;
		assert_eq!(s[s.len() - 1], b'\0', "s has to be nul-terminated!");
		while i < ignore.len() {
			if s[i] == c && ignore[i] as u8 == 0 {
				ignore[i] = true;
				i += 1;
				while i < ignore.len() && s[i] != c {
					ignore[i] = true;
					i += 1;
				}
				if i < ignore.len() && s[i] == c {
					ignore[i] = true;
				}
				while i < ignore.len() && s[i] != c {
					ignore[i] = false;
					i -= 1;
				}
			} else {
				i += 1;
			}
		}
	}
}

impl BoolArray for Box<[bool]> {
	fn zeroing(len: usize) -> Self {
		let mut vec = Vec::with_capacity(len + 1);
		vec.resize(len + 1, false);
		vec.into_boxed_slice()
	}
	fn range_ignore(s: &[u8], ignore: &mut Self, c: u8) {
		let mut i = 0;
		assert_eq!(s[s.len() - 1], b'\0', "s has to be nul-terminated!");
		while i < ignore.len() {
			if s[i] == c && ignore[i] as u8 == 0 {
				ignore[i] = true;
				i += 1;
				while i < ignore.len() && s[i] != c {
					ignore[i] = true;
					i += 1;
				}
				if i < ignore.len() && s[i] == c {
					ignore[i] = true;
				}
				while i < ignore.len() && s[i] != c {
					ignore[i] = false;
					i -= 1;
				}
			} else {
				i += 1;
			}
		}
	}
}

#[allow(unused)]
pub fn bool_arr_zeroing_vec(len: usize) -> Vec<bool> {
	Vec::zeroing(len)
}

#[allow(unused)]
pub fn bool_arr_zeroing_box(len: usize) -> Box<[bool]> {
	Box::zeroing(len)
}

#[cfg(test)]
mod tests {
	#![allow(unused_mut)]
	#![allow(dead_code)]
	#![allow(unsafe_op_in_unsafe_fn)]

	unsafe fn range_ignore_ptr(
		mut s: *const libc::c_char,
		mut ignore: *mut bool,
		mut c: libc::c_uchar,
	) {
		let mut i = 0;
		while *s.add(i) != 0 {
			if *s.add(i) as u8 == c && *ignore.add(i) as u8 == 0 {
				*ignore.add(i) = true;
				i += 1;
				while *s.add(i) != 0 && *s.add(i) as u8 != c {
					*ignore.add(i) = true;
					i += 1;
				}
				if *s.add(i) as u8 == c {
					*ignore.add(i) = true;
				}
				while *s.add(i) as u8 != c {
					*ignore.add(i) = false;
					i -= 1;
				}
			} else {
				i += 1;
			}
		}
	}

	#[allow(dead_code)]
	unsafe fn bool_arr_zeroing(mut len: usize) -> *mut bool {
		let mut i: usize = 0;
		let mut ignore: *mut bool = libc::calloc(len, ::core::mem::size_of::<bool>()) as *mut bool;
		if ignore.is_null() {
			return std::ptr::null_mut::<bool>();
		}
		while i < len.wrapping_add(1) {
			*ignore.add(i) = false;
			i = i.wrapping_add(1);
		}
		ignore
	}

	use libc::{size_t, strlen};

	use super::{BoolArray, bool_arr_zeroing_box, bool_arr_zeroing_vec};

	unsafe fn support_bool_arr_zeroing(mut ignore: *mut *mut bool, mut len: usize) {
		*ignore = bool_arr_zeroing(len) as *mut bool;
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
		arr
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
			let mut s = c"this is my input \"ignore\" 't' 'this' \"is\" 'a' \"test\" 'string'";
			let _expected: *mut bool = support_expected(
				b"00000000000000000111111110111011111101111011101111110111111110\0" as *const u8
					as *const libc::c_char,
			);
			let mut len = libc::strlen(s.as_ptr());
			let mut arr = bool_arr_zeroing(len);
			let mut vec_arr: Vec<bool> = BoolArray::zeroing(len);
			let mut bool_box: Box<[bool]> = BoolArray::zeroing(len);
			if arr.is_null() {
				assert!(false);
			}
			range_ignore_ptr(s.as_ptr(), arr, b'"');
			range_ignore_ptr(s.as_ptr(), arr, b'\'');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'"');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'\'');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'"');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'\'');
			support_ranges_test0(arr);
			support_ranges_test0(vec_arr.as_mut_ptr());
			support_ranges_test0(bool_box.as_mut_ptr());
			libc::free(arr as *mut libc::c_void);
			libc::free(_expected as *mut libc::c_void);
		}
	}
	#[test]
	fn test_range_ignore_sq_0() {
		unsafe {
			let mut arr: *mut bool = std::ptr::null_mut();
			let mut s = c"this is my input 'ignore'";
			let len = libc::strlen(s.as_ptr());
			support_bool_arr_zeroing(&mut arr, len);
			let mut vec_arr = bool_arr_zeroing_vec(len);
			let mut bool_box = bool_arr_zeroing_box(len);
			let mut i = 0;
			while i <= len {
				i += 1;
			}
			range_ignore_ptr(s.as_ptr(), arr, b'\'');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'\'');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'\'');
			let mut i_0 = 0;
			while i_0 <= 16 {
				assert_eq!(false, *arr.add(i_0));
				assert_eq!(false, *vec_arr.as_ptr().add(i_0));
				assert_eq!(false, *bool_box.as_ptr().add(i_0));
				i_0 += 1;
			}
			let mut i_1 = 17;
			while i_1 <= 24 {
				assert_eq!(true, *arr.add(i_1));
				assert_eq!(true, *vec_arr.as_ptr().add(i_1));
				assert_eq!(true, *bool_box.as_ptr().add(i_1));
				i_1 += 1;
			}
			libc::free(arr as *mut libc::c_void);
		}
	}
	#[test]
	fn test_range_ignore_dq_0() {
		unsafe {
			let mut arr: *mut bool = std::ptr::null_mut();
			let mut s = c"this is my input \"ignore\"";
			let len = libc::strlen(s.as_ptr());
			support_bool_arr_zeroing(&mut arr, len);
			let mut vec_arr = bool_arr_zeroing_vec(len);
			let mut bool_box = bool_arr_zeroing_box(len);
			range_ignore_ptr(s.as_ptr(), arr, b'"');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'"');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'"');
			let mut i_0 = 0;
			while i_0 < 17 {
				assert_eq!(false, *arr.add(i_0));
				assert_eq!(false, *vec_arr.as_ptr().add(i_0));
				assert_eq!(false, *bool_box.as_ptr().add(i_0));
				i_0 += 1;
			}
			let mut i_1 = 17;
			while i_1 <= 24 {
				assert_eq!(true, *arr.add(i_0));
				assert_eq!(true, *vec_arr.as_ptr().add(i_0));
				assert_eq!(true, *bool_box.as_ptr().add(i_0));
				i_1 += 1;
			}
			libc::free(arr as *mut libc::c_void);
		}
	}
	#[test]
	fn test_rangeignore_both_0() {
		unsafe {
			let mut arr: *mut bool = std::ptr::null_mut::<bool>();
			let mut s = c"this is my input \"'ignore'\"";
			let len = libc::strlen(s.as_ptr());
			support_bool_arr_zeroing(&mut arr, len);
			let mut vec_arr = bool_arr_zeroing_vec(len);
			let mut bool_box = bool_arr_zeroing_box(len);
			for i in 0..=len {
				assert_eq!(false, *arr.add(i));
				assert_eq!(false, *vec_arr.as_ptr().add(i));
				assert_eq!(false, *bool_box.as_ptr().add(i));
			}
			range_ignore_ptr(s.as_ptr(), arr, b'"');
			range_ignore_ptr(s.as_ptr(), arr, b'\'');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'"');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'\'');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'"');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'\'');
			assert!(*arr.add(24));
			for i in 0..17 {
				assert_eq!(*arr.add(i), false);
			}
			for i in 17..24 {
				assert_eq!(*arr.add(i), true);
			}
			for i in 0..24 {
				assert_eq!(*arr.add(i), *vec_arr.as_ptr().add(i));
				assert_eq!(*arr.add(i), *bool_box.as_ptr().add(i));
			}
			libc::free(arr.cast());
		}
	}
	#[test]
	fn test_ignore_can_work1() {
		unsafe {
			let mut arr: *mut bool = std::ptr::null_mut::<bool>();
			let mut s = c"\"'\"";
			let len = libc::strlen(s.as_ptr());
			support_bool_arr_zeroing(&mut arr, len);
			let mut vec_arr = bool_arr_zeroing_vec(len);
			let mut bool_box = bool_arr_zeroing_box(len);
			range_ignore_ptr(s.as_ptr(), arr, b'\'');
			range_ignore_ptr(s.as_ptr(), arr, b'\"');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'\'');
			// range_ignore_ptr(s.as_ptr(), vec_arr.as_mut_ptr(), b'"');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'\'');
			// range_ignore_ptr(s.as_ptr(), bool_box.as_mut_ptr(), b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut vec_arr, b'"');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'\'');
			BoolArray::range_ignore(s.to_bytes_with_nul(), &mut bool_box, b'"');
			let expected = support_expected(c"1110".as_ptr());
			for i in 0..=len {
				assert_eq!(*expected.add(i), *arr.add(i));
			}
			for i in 0..=len - 1 {
				assert_eq!(*expected.add(i), *vec_arr.as_ptr().add(i));
				assert_eq!(*expected.add(i), *bool_box.as_ptr().add(i));
			}
			libc::free(arr.cast());
			libc::free(expected.cast());
		}
	}
}
