// src/prelude.rs
// other pub use statements

pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
// pub type cc_t = libc::c_uchar;
// pub type speed_t = libc::c_uint;
// pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = i32;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub use crate::eprint_msh;
pub use crate::msh::{e_arg, e_redir, s_func, t_arg, t_shell, t_token};
pub use crate::utils::interop::*;
pub use libc::{c_char, c_int};
