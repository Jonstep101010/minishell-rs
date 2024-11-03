use ::libc;
extern "C" {
	fn ft_isalpha(c: libc::c_int) -> libc::c_int;
	fn ft_isalnum(c: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check_valid_key(mut s: *const libc::c_char) -> bool {
	let mut i: libc::c_int = 0;
	i = 0 as libc::c_int;
	if *s as libc::c_int == '=' as i32
		|| !(ft_isalpha(*s as libc::c_int) != 0 || *s as libc::c_int == '_' as i32)
	{
		return 0 as libc::c_int != 0;
	}
	while *s.offset(i as isize) as libc::c_int != 0
		&& *s.offset(i as isize) as libc::c_int != '=' as i32
	{
		if ft_isalnum(*s.offset(i as isize) as libc::c_int) != 0
			|| *s.offset(i as isize) as libc::c_int == '_' as i32
		{
			i += 1;
		} else {
			return 0 as libc::c_int != 0;
		}
	}
	if *s.offset(i as isize) as libc::c_int == '=' as i32
		&& *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int == '=' as i32
	{
		return 0 as libc::c_int != 0;
	}
	return 1 as libc::c_int != 0;
}
