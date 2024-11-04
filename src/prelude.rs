// src/prelude.rs
// other pub use statements

pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type e_redir = libc::c_uint;
pub const HEREDOC: e_redir = 4;
pub const APPEND: e_redir = 3;
pub const OUTPUT_REDIR: e_redir = 2;
pub const INPUT_REDIR: e_redir = 1;
pub const NO_REDIR: e_redir = 0;
pub type e_arg = libc::c_uint;
pub const REDIR_REMOVED: e_arg = 2;
pub const REDIR: e_arg = 1;
pub const STRING: e_arg = 0;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
