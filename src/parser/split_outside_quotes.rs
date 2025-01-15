use ::libc;
use libc::free;
use libft_rs::{
	ft_strchr::ft_strchr, ft_strlen::ft_strlen, ft_strtrim::ft_strtrim, ft_substr::ft_substr,
};
use libutils_rs::src::array::append_str::append_str_arr_free;

use crate::size_t;

#[derive(Copy, Clone)]
#[repr(C)]
struct t_splitter {
	pub quote: libc::c_int,
	pub start: size_t,
	pub len: size_t,
	pub arr: *mut *mut libc::c_char,
	pub to_split: *mut libc::c_char,
	pub set: *const libc::c_char,
}
unsafe fn split_loop(mut s: *mut t_splitter) -> *mut *mut libc::c_char {
	let mut i: size_t = 0;
	while i < (*s).len {
		if (*s).quote != 0 && *((*s).to_split).offset(i as isize) as libc::c_int == (*s).quote {
			(*s).quote = 0 as libc::c_int;
		} else if (*s).quote == 0
			&& !(ft_strchr(
				b"'\"\0" as *const u8 as *const libc::c_char,
				*((*s).to_split).offset(i as isize) as libc::c_int,
			))
			.is_null()
		{
			(*s).quote = *((*s).to_split).offset(i as isize) as libc::c_int;
		}
		if (*s).quote == 0
			&& !(ft_strchr((*s).set, *((*s).to_split).offset(i as isize) as libc::c_int)).is_null()
		{
			(*s).arr = append_str_arr_free(
				(*s).arr,
				ft_substr(
					(*s).to_split,
					(*s).start as libc::c_uint,
					i.wrapping_sub((*s).start),
				),
			);
			if ((*s).arr).is_null() {
				return std::ptr::null_mut::<*mut libc::c_char>();
			}
			while *((*s).to_split)
				.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
				as libc::c_int
				!= 0 && !(ft_strchr(
				(*s).set,
				*((*s).to_split).offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
					as libc::c_int,
			))
			.is_null()
			{
				i = i.wrapping_add(1);
			}
			(*s).start = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
		}
		i = i.wrapping_add(1);
	}
	append_str_arr_free(
		(*s).arr,
		ft_substr(
			&*((*s).to_split).offset((*s).start as isize),
			0 as libc::c_int as libc::c_uint,
			i.wrapping_sub((*s).start),
		),
	)
}

pub unsafe fn split_outside_quotes(
	mut to_split: *const libc::c_char,
	mut set: *const libc::c_char,
) -> *mut *mut libc::c_char {
	if to_split.is_null() {
		return std::ptr::null_mut::<*mut libc::c_char>();
	}
	let mut s = {
		t_splitter {
			quote: 0 as libc::c_int,
			start: 0 as libc::c_int as size_t,
			len: 0 as libc::c_int as size_t,
			arr: std::ptr::null_mut::<*mut libc::c_char>(),
			to_split: ft_strtrim(to_split, set),
			set,
		}
	};
	if (s.to_split).is_null() {
		return std::ptr::null_mut::<*mut libc::c_char>();
	}
	s.len = ft_strlen(s.to_split);
	let mut ret: *mut *mut libc::c_char = split_loop(&mut s);
	free(s.to_split as *mut libc::c_void);
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
			let mut vec_output = charptr_array_to_vec(output);
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
			let mut vec_output = charptr_array_to_vec(output);
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
			let mut vec_output = charptr_array_to_vec(output);
			dbg!(&vec_output);
			assert_eq!(expected, vec_output);
			let mut trim = ft_strtrim(*output, c" \t\n\r".as_ptr());
			let output_two = split_outside_quotes(trim, c" \t\n\r".as_ptr());
			let mut vec_output = charptr_array_to_vec(output_two);
			assert_eq!(expected_two, vec_output);
			libutils_rs::arr_free(output);
			libutils_rs::arr_free(output_two);
			libc::free(trim.cast());
		}
	}
}
