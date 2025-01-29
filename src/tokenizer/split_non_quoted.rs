pub(super) fn split_non_quoted(to_split: &str, set: &str) -> Vec<String> {
	let to_split = to_split.trim_matches(|c| set.contains(c));
	let bytes = to_split.as_bytes();
	let len = bytes.len();
	let mut quote = 0;
	let mut start = 0;
	let mut vec_splits = vec![];
	let mut i = 0;
	while i < len {
		quote = if quote != 0 && bytes[i] == quote {
			0
		} else if quote == 0 && (bytes[i] == b'\'' || bytes[i] == b'"') {
			bytes[i]
		} else {
			quote
		};
		if quote == 0 && set.as_bytes().contains(&bytes[i]) {
			let string_slice = &to_split[start..i];
			vec_splits.push(string_slice.to_string());
			while set.as_bytes().contains(&bytes[i + 1]) {
				i += 1;
			}
			start = i + 1;
		}
		i += 1;
	}
	let last = &to_split[start..i];
	vec_splits.push(last.to_string());
	vec_splits
}

// pub unsafe fn split_outside_quotes(
// 	to_split: *const libc::c_char,
// 	set: *const libc::c_char,
// ) -> *mut *mut libc::c_char {
// 	use libft_rs::ft_substr::ft_substr;
// 	if to_split.is_null() {
// 		return std::ptr::null_mut::<*mut libc::c_char>();
// 	}
// 	let to_split = libft_rs::ft_strtrim::ft_strtrim(to_split, set);
// 	if (to_split).is_null() {
// 		return std::ptr::null_mut::<*mut libc::c_char>();
// 	}
// 	let cstr_to_split = std::ffi::CStr::from_ptr(to_split);
// 	let len = cstr_to_split.count_bytes();
// 	let bytes = cstr_to_split.to_bytes();
// 	let mut quote = 0;
// 	let mut start: usize = 0;
// 	let mut vec_arr = vec![];
// 	let mut i: usize = 0;
// 	while i < len {
// 		if quote != 0 && bytes[i] == quote {
// 			quote = 0;
// 		} else if quote == 0 && (bytes[i] == b'\'' || bytes[i] == b'"') {
// 			quote = bytes[i];
// 		}
// 		if quote == 0 && !(libft_rs::ft_strchr::ft_strchr(set, bytes[i].into())).is_null() {
// 			let str = ft_substr(to_split, start as u32, i.wrapping_sub(start) as u64);
// 			vec_arr.push(str);
// 			while bytes[i.wrapping_add(1)] != 0
// 				&& !(libft_rs::ft_strchr::ft_strchr(set, bytes[i.wrapping_add(1)].into())).is_null()
// 			{
// 				i = i.wrapping_add(1);
// 			}
// 			start = i.wrapping_add(1);
// 		}
// 		i = i.wrapping_add(1);
// 	}
// 	let str = ft_substr(&*(to_split).add(start), 0, i.wrapping_sub(start) as u64);
// 	vec_arr.push(str);
// 	libc::free(to_split as *mut libc::c_void);
// 	let ret = libft_rs::ft_calloc(
// 		vec_arr.len().wrapping_add(2) as u64,
// 		::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
// 	) as *mut *mut libc::c_char;
// 	for (i, vec_item) in vec_arr.iter().enumerate() {
// 		*ret.add(i) = *vec_item;
// 	}
// 	ret
// }

#[cfg(test)]
mod tests {
	use rstest::{fixture, rstest};

	// ///
	// /// # Safety
	// ///
	// /// the caller must free the input and ensure it is null terminated
	// ///
	// /// since the return contains borrowed values,
	// /// `array` cannot be freed until `vec_output` is no longer being used
	// #[allow(unused_mut)]
	// unsafe fn charptr_array_to_vec<'a>(array: *mut *mut libc::c_char) -> Vec<&'a str> {
	// 	assert!(!array.is_null());
	// 	let mut output_ptr = array;
	// 	let mut vec_output = vec![];
	// 	while !(*output_ptr).is_null() {
	// 		let mut elem = std::ffi::CStr::from_ptr(*output_ptr);
	// 		vec_output.push(
	// 			elem.to_str()
	// 				.expect("should be non null and valid for pushing to vec!"),
	// 		);
	// 		output_ptr = output_ptr.add(1);
	// 	}
	// 	vec_output
	// }
	// use libft_rs::ft_strtrim;
	// use super::split_outside_quotes;

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
		// let cstr = std::ffi::CString::new(input).unwrap();
		// unsafe {
		// 	let output = split_outside_quotes(cstr.as_ptr(), c"|".as_ptr());
		// 	let vec_output = charptr_array_to_vec(output);
		// dbg!(&vec_output);
		// assert_eq!(expected, vec_output);
		let vec_safe_output = super::split_non_quoted(input, "|");
		assert_eq!(expected, vec_safe_output);
		// libutils_rs::arr_free(output);
		// }
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
		// let cstr = std::ffi::CString::new(input).unwrap();
		// unsafe {
		// 	let output = split_outside_quotes(cstr.as_ptr(), c" \t\n\r".as_ptr());
		// 	let vec_output = charptr_array_to_vec(output);
		// 	dbg!(&vec_output);
		// 	assert_eq!(expected, vec_output);
		let vec_safe_output = super::split_non_quoted(input, " \t\n\r");
		assert_eq!(expected, vec_safe_output);
		// libutils_rs::arr_free(output);
		// }
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
		// let cstr = std::ffi::CString::new(input).unwrap();
		// unsafe {
		// let output = split_outside_quotes(cstr.as_ptr(), c"|".as_ptr());
		// let vec_output = charptr_array_to_vec(output);
		// dbg!(&vec_output);
		// assert_eq!(expected, vec_output);
		let vec_safe_output = super::split_non_quoted(input, "|");
		assert_eq!(expected, vec_safe_output);
		// let trim = ft_strtrim(*output, c" \t\n\r".as_ptr());
		// let output_two = split_outside_quotes(trim, c" \t\n\r".as_ptr());
		// libc::free(trim.cast());
		// let vec_output = charptr_array_to_vec(output_two);
		// assert_eq!(expected_two, vec_output);
		let trim = vec_safe_output[0].trim_matches(|c| " \t\n\r".contains(c));
		let vec_safe_output_two = super::split_non_quoted(trim, " \t\n\r");
		assert_eq!(expected_two, vec_safe_output_two);
		// libutils_rs::arr_free(output);
		// libutils_rs::arr_free(output_two);
		// }
	}
}
