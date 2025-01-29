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

#[cfg(test)]
mod tests {
	use rstest::{fixture, rstest};

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
		let vec_safe_output = super::split_non_quoted(input, "|");
		assert_eq!(expected, vec_safe_output);
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
		let vec_safe_output = super::split_non_quoted(input, " \t\n\r");
		assert_eq!(expected, vec_safe_output);
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
		let vec_safe_output = super::split_non_quoted(input, "|");
		assert_eq!(expected, vec_safe_output);
		let trim = vec_safe_output[0].trim_matches(|c| " \t\n\r".contains(c));
		let vec_safe_output_two = super::split_non_quoted(trim, " \t\n\r");
		assert_eq!(expected_two, vec_safe_output_two);
	}
}
