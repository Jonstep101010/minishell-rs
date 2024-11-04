use ::libc;
use libc::write;
use libftprintf_rs::ft_vdprintf::ft_vdprintf;
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
	pub __stack: *mut libc::c_void,
	pub __gr_top: *mut libc::c_void,
	pub __vr_top: *mut libc::c_void,
	pub __gr_offs: libc::c_int,
	pub __vr_offs: libc::c_int,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[no_mangle]
pub unsafe extern "C" fn eprint(mut fmt: *const libc::c_char, mut args: ...) {
	let mut args_0: ::core::ffi::VaListImpl;
	args_0 = args.clone();
	write(
		2 as libc::c_int,
		b"minishell: \0" as *const u8 as *const libc::c_char as *const libc::c_void,
		11,
	);
	ft_vdprintf(2 as libc::c_int, fmt, args_0.as_va_list());
	write(
		2 as libc::c_int,
		b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
		1,
	);
}
#[no_mangle]
pub unsafe extern "C" fn eprint_single(mut fmt: *const libc::c_char, mut args: ...) {
	let mut args_0: ::core::ffi::VaListImpl;
	args_0 = args.clone();
	ft_vdprintf(2 as libc::c_int, fmt, args_0.as_va_list());
}
