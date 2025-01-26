// pub use i8const_str;

#[must_use]
pub unsafe fn i8const_str(command: *const *const i8, i: u64) -> &'static str {
	let c_ptr_char: *const std::os::raw::c_char = *command.offset(i as isize);
	let faulty_identifier: &std::ffi::CStr = std::ffi::CStr::from_ptr(c_ptr_char);
	let faulty_identifier: &str = faulty_identifier.to_str().unwrap();
	faulty_identifier
}

///
/// # Safety
///
/// the caller must free the input and ensure it is null terminated
///
/// since the return contains borrowed values,
/// `array` cannot be freed until `vec_output` is no longer being used
#[allow(unused_mut)]
pub unsafe fn charptr_array_to_vec<'a>(array: *mut *mut libc::c_char) -> Vec<&'a str> {
	assert!(!array.is_null());
	let mut output_ptr = array;
	let mut vec_output = vec![];
	while !(*output_ptr).is_null() {
		let mut elem = std::ffi::CStr::from_ptr(*output_ptr);
		vec_output.push(
			elem.to_str()
				.expect("should be non null and valid for pushing to vec!"),
		);
		output_ptr = output_ptr.add(1);
	}
	vec_output
}
