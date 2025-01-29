// src/prelude.rs
// other pub use statements

pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = i32;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub use crate::environment::Env;
pub use crate::eprint_msh;
pub use crate::msh::{e_arg::*, e_redir::*, t_arg, t_shell, t_token};
pub use std::ffi::CString;
