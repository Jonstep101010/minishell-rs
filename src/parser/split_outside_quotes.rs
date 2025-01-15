#![deny(unused_mut)]

use ::libc;
use libc::{c_char, c_int, free};
use libft_rs::{
	ft_strchr::ft_strchr, ft_strlen::ft_strlen, ft_strtrim::ft_strtrim, ft_substr::ft_substr,
};
use libutils_rs::src::array::append_str::append_str_arr_free;

pub unsafe fn split_outside_quotes(
	to_split: *const c_char,
	set: *const c_char,
) -> *mut *mut c_char {
	if to_split.is_null() {
		return std::ptr::null_mut::<*mut c_char>();
	}
	let to_split = ft_strtrim(to_split, set);
	if (to_split).is_null() {
		return std::ptr::null_mut::<*mut c_char>();
	}
	let len = ft_strlen(to_split) as usize;
	let mut quote = 0;
	let mut start = 0;
	let mut arr = std::ptr::null_mut::<*mut c_char>();
	let mut i: usize = 0;
	while i < len {
		if quote != 0 && *(to_split).add(i) as c_int == quote {
			quote = 0 as c_int;
		} else if quote == 0 && !(ft_strchr(c"'\"".as_ptr(), *(to_split).add(i) as c_int)).is_null()
		{
			quote = *(to_split).add(i) as c_int;
		}
		if quote == 0 && !(ft_strchr(set, *(to_split).add(i) as c_int)).is_null() {
			arr = append_str_arr_free(
				arr,
				ft_substr(
					to_split,
					start as libc::c_uint,
					i.wrapping_sub(start) as u64,
				),
			);
			if (arr).is_null() {
				return std::ptr::null_mut::<*mut c_char>();
			}
			while *(to_split).add(i.wrapping_add(1)) as c_int != 0
				&& !(ft_strchr(set, *(to_split).add(i.wrapping_add(1)) as c_int)).is_null()
			{
				i = i.wrapping_add(1);
			}
			start = i.wrapping_add(1);
		}
		i = i.wrapping_add(1);
	}
	let ret = append_str_arr_free(
		arr,
		ft_substr(
			&*(to_split).add(start),
			0 as c_int as libc::c_uint,
			i.wrapping_sub(start) as u64,
		),
	);
	free(to_split as *mut libc::c_void);
	ret
}

#[cfg(test)]
mod tests {
	use libft_rs::ft_strtrim;
	use rstest::{fixture, rstest};

	use crate::charptr_array_to_vec;

	use super::split_outside_quotes;

	#[rstest]
	#[case(vec!["echo ", " \"nopipes |\" ", " echo hello"], "echo | \"nopipes |\" | echo hello")]
	#[case(vec!["echo"], "echo")]
	#[case(vec!["$somedir "], "$somedir ")]
	#[case(vec!["echo $somedir"], "||echo $somedir|")]
	#[case(vec![">tmp_out ", " echo 1"], ">tmp_out | echo 1")]
	#[case(vec!["cat << delim ", " cat"], "cat << delim | cat")]
	#[case(vec!["cat ", " cat ", " ls"], "cat | cat | ls")]
	#[case(vec!["echo hello    ", " echo world ", " cat"], "echo hello    || echo world | cat")]
	#[case(vec!["cat ", " ls"], "cat | ls")]
	#[fixture]
	fn test_split_pipes(#[case] expected: Vec<&str>, #[case] input: &str) {
		let cstr = std::ffi::CString::new(input).unwrap();
		unsafe {
			let output = split_outside_quotes(cstr.as_ptr(), c"|".as_ptr());
			let vec_output = charptr_array_to_vec(output);
			dbg!(&vec_output);
			assert_eq!(expected, vec_output);
			libutils_rs::arr_free(output);
		}
	}
	#[rstest]
	#[case(vec!["echo", "$somedir' '"], "         echo $somedir' '           ")]
	#[case(vec![""], "                    ")]
	#[case(vec!["h"], "        h            ")]
	#[case(vec!["'"], "        '            ")]
	#[case(vec!["'h'"], "        'h'            ")]
	#[case(vec!["h"], "    \t \r   h   \n  \t  \r  \n  ")]
	#[case(vec!["echo", "hello", "world", "'>'", "file", "<", "file2"], "echo hello world '>' file < file2")]
	#[case(vec!["cat", "<<", "delim"], "cat << delim ")]
	#[case(vec!["cat"], "cat ")]
	#[case(vec!["cat"], " cat ")]
	#[case(vec!["ls"], " ls")]
	#[case(vec!["echo", "hello"], "echo hello    ")]
	#[case(vec!["echo", "world"], "echo world ")]
	#[fixture]
	fn test_split_whitespace(#[case] expected: Vec<&str>, #[case] input: &str) {
		let cstr = std::ffi::CString::new(input).unwrap();
		unsafe {
			let output = split_outside_quotes(cstr.as_ptr(), c" \t\n\r".as_ptr());
			let vec_output = charptr_array_to_vec(output);
			dbg!(&vec_output);
			assert_eq!(expected, vec_output);
			libutils_rs::arr_free(output);
		}
	}
	#[rstest]
	#[case(vec!["ls \n-l\r \tsomedir ", " cat -e ", " wc -l"], vec!["ls", "-l", "somedir"], "ls \n-l\r \tsomedir | cat -e | wc -l")]
	#[case(vec!["ls -l somedir ", " cat -e ", " wc -l"], vec!["ls", "-l", "somedir"], "ls -l somedir | cat -e | wc -l")]
	#[fixture]
	fn test_split_pipes_whitespace(
		#[case] expected: Vec<&str>,
		#[case] expected_two: Vec<&str>,
		#[case] input: &str,
	) {
		let cstr = std::ffi::CString::new(input).unwrap();
		unsafe {
			let output = split_outside_quotes(cstr.as_ptr(), c"|".as_ptr());
			let vec_output = charptr_array_to_vec(output);
			dbg!(&vec_output);
			assert_eq!(expected, vec_output);
			let trim = ft_strtrim(*output, c" \t\n\r".as_ptr());
			let output_two = split_outside_quotes(trim, c" \t\n\r".as_ptr());
			let vec_output = charptr_array_to_vec(output_two);
			assert_eq!(expected_two, vec_output);
			libutils_rs::arr_free(output);
			libutils_rs::arr_free(output_two);
			libc::free(trim.cast());
		}
	}
}
