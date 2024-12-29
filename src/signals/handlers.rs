use ::libc;
extern "C" {
	fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
	fn sigaction(
		__sig: libc::c_int,
		__act: *const sigaction,
		__oact: *mut sigaction,
	) -> libc::c_int;
	fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
	fn tcsetattr(
		__fd: libc::c_int,
		__optional_actions: libc::c_int,
		__termios_p: *const termios,
	) -> libc::c_int;
	static mut g_ctrl_c: libc::c_int;
}
use gnu_readline_sys::{rl_on_new_line, rl_redisplay, rl_replace_line};
use libc::write;

use crate::{prelude::*, termios};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
	pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
	pub sival_int: libc::c_int,
	pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
	pub si_signo: libc::c_int,
	pub si_errno: libc::c_int,
	pub si_code: libc::c_int,
	pub __pad0: libc::c_int,
	pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
	pub _pad: [libc::c_int; 28],
	pub _kill: C2RustUnnamed_8,
	pub _timer: C2RustUnnamed_7,
	pub _rt: C2RustUnnamed_6,
	pub _sigchld: C2RustUnnamed_5,
	pub _sigfault: C2RustUnnamed_2,
	pub _sigpoll: C2RustUnnamed_1,
	pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
	pub _call_addr: *mut libc::c_void,
	pub _syscall: libc::c_int,
	pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
	pub si_band: libc::c_long,
	pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
	pub si_addr: *mut libc::c_void,
	pub si_addr_lsb: libc::c_short,
	pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
	pub _addr_bnd: C2RustUnnamed_4,
	pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
	pub _lower: *mut libc::c_void,
	pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
	pub si_pid: __pid_t,
	pub si_uid: __uid_t,
	pub si_status: libc::c_int,
	pub si_utime: __clock_t,
	pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
	pub si_pid: __pid_t,
	pub si_uid: __uid_t,
	pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
	pub si_tid: libc::c_int,
	pub si_overrun: libc::c_int,
	pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
	pub si_pid: __pid_t,
	pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
	pub __sigaction_handler: C2RustUnnamed_10,
	pub sa_mask: __sigset_t,
	pub sa_flags: libc::c_int,
	pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
	pub sa_handler: __sighandler_t,
	pub sa_sigaction:
		Option<unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ()>,
}
#[no_mangle]
pub unsafe extern "C" fn check_signals(mut p_termios: *mut termios) {
	tcgetattr(0 as libc::c_int, p_termios);
	(*p_termios).c_lflag &= !(0o1000 as libc::c_int) as libc::c_uint;
	tcsetattr(0 as libc::c_int, 2 as libc::c_int, p_termios);
	ctrl_c_init();
	ctrl_bkslash_init();
}
unsafe extern "C" fn ctrl_bkslash_init() {
	let mut sig: sigaction = sigaction {
		__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
		sa_mask: __sigset_t { __val: [0; 16] },
		sa_flags: 0,
		sa_restorer: None,
	};
	sig.__sigaction_handler.sa_handler = ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
		1 as libc::c_int as libc::intptr_t,
	);
	sig.sa_flags = 0x10000000 as libc::c_int;
	sigemptyset(&mut sig.sa_mask);
	sigaction(3 as libc::c_int, &sig, std::ptr::null_mut::<sigaction>());
}
unsafe extern "C" fn ctrl_c_init() {
	let mut sig: sigaction = sigaction {
		__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
		sa_mask: __sigset_t { __val: [0; 16] },
		sa_flags: 0,
		sa_restorer: None,
	};
	sig.__sigaction_handler.sa_sigaction = Some(
		ctrl_c_handler
			as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
	);
	sig.sa_flags = 0x10000000 as libc::c_int;
	sigemptyset(&mut sig.sa_mask);
	sigaction(2 as libc::c_int, &sig, std::ptr::null_mut::<sigaction>());
}
unsafe extern "C" fn ctrl_c_handler(
	mut sig: libc::c_int,
	mut _info: *mut siginfo_t,
	mut _unused: *mut libc::c_void,
) {
	if sig == 2 as libc::c_int {
		g_ctrl_c = 1 as libc::c_int;
		write(
			0 as libc::c_int,
			b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
			1,
		);
		rl_on_new_line();
		rl_replace_line(b"\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
		rl_redisplay();
	}
}

#[no_mangle]
pub unsafe extern "C" fn check_signals_child(mut p_termios_child: *mut termios) {
	let mut attr: termios = termios {
		c_iflag: 0,
		c_oflag: 0,
		c_cflag: 0,
		c_lflag: 0,
		c_line: 0,
		c_cc: [0; 32],
		c_ispeed: 0,
		c_ospeed: 0,
	};
	tcgetattr(0 as libc::c_int, p_termios_child);
	tcgetattr(0 as libc::c_int, &mut attr);
	attr.c_lflag &= !(0o1000 as libc::c_int) as libc::c_uint;
	tcsetattr(1 as libc::c_int, 2 as libc::c_int, p_termios_child);
	ctrl_backslash_child();
	ctrl_c_child();
}
unsafe extern "C" fn ctrl_c_child() {
	let mut ctrl_c: sigaction = sigaction {
		__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
		sa_mask: __sigset_t { __val: [0; 16] },
		sa_flags: 0,
		sa_restorer: None,
	};
	ctrl_c.__sigaction_handler.sa_handler = None;
	ctrl_c.sa_flags = 0x10000000 as libc::c_int;
	sigemptyset(&mut ctrl_c.sa_mask);
	sigaction(2 as libc::c_int, &ctrl_c, std::ptr::null_mut::<sigaction>());
}
unsafe extern "C" fn ctrl_backslash_child() {
	let mut ctrl_slash: sigaction = sigaction {
		__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
		sa_mask: __sigset_t { __val: [0; 16] },
		sa_flags: 0,
		sa_restorer: None,
	};
	ctrl_slash.__sigaction_handler.sa_handler = None;
	ctrl_slash.sa_flags = 0x10000000 as libc::c_int;
	sigemptyset(&mut ctrl_slash.sa_mask);
	sigaction(
		3 as libc::c_int,
		&ctrl_slash,
		std::ptr::null_mut::<sigaction>(),
	);
}
