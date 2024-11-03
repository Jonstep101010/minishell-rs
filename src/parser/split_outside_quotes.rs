use ::libc;
extern "C" {
	fn free(_: *mut libc::c_void);
	fn ft_strlen(str: *const libc::c_char) -> size_t;
	fn ft_strchr(str: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
	fn ft_substr(s: *const libc::c_char, start: libc::c_uint, len: size_t) -> *mut libc::c_char;
	fn ft_strtrim(s1: *const libc::c_char, set: *const libc::c_char) -> *mut libc::c_char;
	fn append_str_arr_free(
		arr: *mut *mut libc::c_char,
		s: *mut libc::c_char,
	) -> *mut *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_splitter {
	pub quote: libc::c_int,
	pub start: size_t,
	pub len: size_t,
	pub arr: *mut *mut libc::c_char,
	pub to_split: *mut libc::c_char,
	pub set: *const libc::c_char,
}
pub type t_splitter = s_splitter;
unsafe extern "C" fn split_loop(mut s: *mut t_splitter) -> *mut *mut libc::c_char {
	let mut i: size_t = 0;
	i = 0 as libc::c_int as size_t;
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
				return 0 as *mut *mut libc::c_char;
			}
			while *((*s).to_split)
				.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
				!= 0 && !(ft_strchr(
				(*s).set,
				*((*s).to_split).offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
					as libc::c_int,
			))
			.is_null()
			{
				i = i.wrapping_add(1);
				i;
			}
			(*s).start = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
		}
		i = i.wrapping_add(1);
		i;
	}
	return append_str_arr_free(
		(*s).arr,
		ft_substr(
			&mut *((*s).to_split).offset((*s).start as isize),
			0 as libc::c_int as libc::c_uint,
			i.wrapping_sub((*s).start),
		),
	);
}
#[no_mangle]
pub unsafe extern "C" fn split_outside_quotes(
	mut to_split: *const libc::c_char,
	mut set: *const libc::c_char,
) -> *mut *mut libc::c_char {
	let mut ret: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
	let mut s: t_splitter = t_splitter {
		quote: 0,
		start: 0,
		len: 0,
		arr: 0 as *mut *mut libc::c_char,
		to_split: 0 as *mut libc::c_char,
		set: 0 as *const libc::c_char,
	};
	if to_split.is_null() {
		return 0 as *mut *mut libc::c_char;
	}
	s = {
		let mut init = s_splitter {
			quote: 0 as libc::c_int,
			start: 0 as libc::c_int as size_t,
			len: 0 as libc::c_int as size_t,
			arr: 0 as *mut *mut libc::c_char,
			to_split: ft_strtrim(to_split, set),
			set: set,
		};
		init
	};
	if (s.to_split).is_null() {
		return 0 as *mut *mut libc::c_char;
	}
	s.len = ft_strlen(s.to_split);
	ret = split_loop(&mut s);
	free(s.to_split as *mut libc::c_void);
	return ret;
}
