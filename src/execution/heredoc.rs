use std::ffi::CString;

use ::libc;
use libc::{close, dup2};

use libft_rs::ft_putendl_fd::ft_putendl_fd;
use libutils_rs::src::string::str_equal::equal;
use nix::{fcntl::OFlag, sys::stat::Mode};

use crate::environment::Env;
use crate::{e_redir, prelude::*, t_token};

#[unsafe(no_mangle)]
pub static mut g_ctrl_c: libc::c_int = 0 as libc::c_int;
#[unsafe(no_mangle)]
unsafe extern "C" fn heredoc_loop(mut delim: *mut libc::c_char, mut fd: libc::c_int, env: &Env) {
	g_ctrl_c = 0 as libc::c_int;
	while 1 as libc::c_int != 0 && g_ctrl_c == 0 {
		let line = crate::utils::rust_readline::str_readline("> ");
		if line.is_none() {
			g_ctrl_c = 0 as libc::c_int;
			break;
		}
		let line = line.unwrap();
		let cstr_line = CString::new(line).unwrap();
		if !(equal(delim, cstr_line.as_ptr())).is_null() || g_ctrl_c != 0 {
			g_ctrl_c = 0 as libc::c_int;
			break;
		}
		if let Some(expanded) = crate::environment::expander::expander(&cstr_line, env) {
			if (equal(expanded.as_ptr(), cstr_line.as_ptr())).is_null() {
				let raw_ptr = expanded.into_raw();
				ft_putendl_fd(raw_ptr, fd);
				let _ = CString::from_raw(raw_ptr);
			} else {
				let raw = cstr_line.into_raw();
				ft_putendl_fd(raw, fd);
				let _ = CString::from_raw(raw);
			}
		}
	}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_heredocs(
	mut token: *mut t_token,
	mut target: *mut libc::c_int,
	env: &Env,
) {
	let mut i: libc::c_int = -1;
	loop {
		i += 1;
		if ((*((*token).cmd_args).offset(i as isize)).elem).is_null() {
			break;
		}
		if (*((*token).cmd_args).offset(i as isize)).redir as libc::c_uint
			== e_redir::HEREDOC as libc::c_int as libc::c_uint
		{
			let oflags = OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC;
			let mode = Mode::from_bits(0o644).expect("Invalid mode");
			match nix::fcntl::open(c".heredoc.txt", oflags, mode) {
				Ok(mut fd) => {
					heredoc_loop((*((*token).cmd_args).offset(i as isize)).elem, fd, env);
					close(fd);
					fd = nix::fcntl::open(c".heredoc.txt", OFlag::O_RDONLY, Mode::empty()).unwrap();
					dup2(fd, *target);
					close(fd);
					let _ = nix::unistd::unlink(c".heredoc.txt");
				}
				Err(e) => {
					eprint_msh!("{}", e);
					panic!("i/o error");
				}
			}
		}
	}
}
