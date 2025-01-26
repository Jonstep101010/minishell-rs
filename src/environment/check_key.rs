use crate::prelude::*;
use libft_rs::{ft_isalnum::ft_isalnum, ft_isalpha::ft_isalpha};

pub unsafe fn check_valid_key(s: *const c_char) -> bool {
	let mut i = 0;
	let s: *const u8 = s.cast();
	if *s == b'=' || !(ft_isalpha(*s as c_int) != 0 || *s == b'_') {
		return false;
	}
	while *s.add(i) != 0 && *s.add(i) != b'=' {
		if ft_isalnum(*s.add(i) as c_int) != 0 || *s.add(i) == b'_' {
			i += 1;
		} else {
			return false;
		}
	}
	!(*s.add(i) == b'=' && *s.add(i + 1) == b'=')
}
