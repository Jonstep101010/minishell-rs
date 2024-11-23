use msh::lexer::*;

#[test]
fn test_result_ok() {
	let inputs = vec![
		"ls -la | grep 'txt'",
		"echo \"Hello World\" > output.txt",
		"echo 'Append this' >> file.txt",
		"> tmpfile > midfile",
		">$",
		"cat << delim | > tmp_out",
		"> infile | cat",
		"<infile | cat",
		"echo > outfile",
		"echo < outfile",
		"> outfile",
		"< outfile",
	];
	for input in inputs {
		let mut lexer = Lexer::new(input);
		let result = lexer.tokenize();
		assert!(result.is_ok());
	}
}

#[test]
fn test_result_err() {
	let inputs = vec![
		"> |",                   // `|'
		"< |",                   // `|'
		"|",                     // `|'
		" | hello",              // `|'
		"< <",                   // `<'
		"< < <",                 // `<'
		"> >",                   // `>'
		"> > >",                 // `>'
		"<",                     // `newline'
		">",                     // `newline'
		"< ",                    // `newline'
		"> ",                    // `newline'
		">>",                    // `newline'
		"<<",                    // `newline'
		">|",                    // `newline'
		"<|",                    // `newline'
		"> tmpfile > midfile >", // `newline'
		"tmpfile > midfile >",   // `newline'
	];
	let expected = vec![
		"|", "|", "|", "|", "<", "<", ">", ">", "newline", "newline", "newline", "newline",
		"newline", "newline", "newline", "newline", "newline", "newline",
	];
	for (input, expected) in inputs.iter().zip(expected.iter()) {
		let mut lexer = Lexer::new(input);
		let result = lexer.tokenize();
		assert_eq!(result, Err(LexerError::UnexpectedToken(expected)));
	}
}
