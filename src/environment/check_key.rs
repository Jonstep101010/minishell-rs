pub fn check_valid_key(s: &[u8]) -> bool {
	let mut i = 0;
	if s[0] == b'=' || !(matches!(s[0] as char,'A'..='Z' | 'a'..='z' | '_')) {
		return false;
	}
	while s[i] != 0 && s[i] != b'=' {
		if matches!(s[i] as char,'A'..='Z' | 'a'..='z' | '0'..='9' | '_') {
			i += 1;
		} else {
			return false;
		}
	}
	!(s[i] == b'=' && s[i + 1] == b'=')
}
