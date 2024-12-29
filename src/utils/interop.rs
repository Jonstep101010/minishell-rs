// pub use i8const_str;

#[must_use]
pub unsafe fn i8const_str(command: *const *const i8, i: u64) -> &'static str {
	let c_ptr_char: *const std::os::raw::c_char = *command.offset(i as isize);
	let faulty_identifier: &std::ffi::CStr = std::ffi::CStr::from_ptr(c_ptr_char);
	let faulty_identifier: &str = faulty_identifier.to_str().unwrap();
	faulty_identifier
}
