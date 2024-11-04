use ::libc;
use libc::free;
use libft_rs::{
	ft_strchr::ft_strchr, ft_strlen::ft_strlen, ft_strtrim::ft_strtrim, ft_substr::ft_substr,
};
use libutils_rs::src::array::append_str::append_str_arr_free;

use crate::size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_splitter {
	pub quote: libc::c_int,
	pub start: size_t,
	pub len: size_t,
	pub arr: *mut *mut libc::c_char,
	pub to_split: *mut libc::c_char,
	pub set: *const libc::c_char,
}
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
#[no_mangle]
pub unsafe extern "C" fn split_outside_quotes(
	mut to_split: *const libc::c_char,
	mut set: *const libc::c_char,
) -> *mut *mut libc::c_char {
	let mut ret: *mut *mut libc::c_char = std::ptr::null_mut::<*mut libc::c_char>();
	let mut s: t_splitter = t_splitter {
		quote: 0,
		start: 0,
		len: 0,
		arr: std::ptr::null_mut::<*mut libc::c_char>(),
		to_split: std::ptr::null_mut::<libc::c_char>(),
		set: std::ptr::null::<libc::c_char>(),
	};
	if to_split.is_null() {
		return std::ptr::null_mut::<*mut libc::c_char>();
	}
	s = {
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
	ret = split_loop(&mut s);
	free(s.to_split as *mut libc::c_void);
	ret
}
