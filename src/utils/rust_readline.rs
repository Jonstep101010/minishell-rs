// The MIT License (MIT)

// Copyright (c) 2014 Sean Perry

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// see https://github.com/shaleh/rust-readline for source (adapted)
/// Invoke the external `readline()`.
///
/// Returns an `Option<String>` representing whether a `String` was returned
/// or NULL. `None` indicates the user has signal end of input.
pub fn str_readline(prompt: &str) -> Option<String> {
	let cprmt = std::ffi::CString::new(prompt).unwrap();
	unsafe {
		let ret = gnu_readline_sys::readline(cprmt.as_ptr());
		if ret.is_null() {
			// user pressed Ctrl-D
			None
		} else {
			let slice = std::ffi::CStr::from_ptr(ret);
			let bytes = slice.to_bytes();

			// the return from readline needs to be explicitly freed
			// so clone the input first
			let line = String::from_utf8_lossy(bytes).into_owned().clone();

			libc::free(ret as *mut libc::c_void);

			Some(line)
		}
	}
}

pub fn str_add_history(line: &str) {
	unsafe {
		gnu_readline_sys::add_history(std::ffi::CString::new(line).unwrap().as_ptr());
	}
}
