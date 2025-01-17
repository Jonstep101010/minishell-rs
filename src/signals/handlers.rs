// use ::libc;
// unsafe {
// 	fn sigemptyset(__set: *mut sigset_t) -> i32;
// 	fn sigaction(
// 		__sig: i32,
// 		__act: *const sigaction,
// 		__oact: *mut sigaction,
// 	) -> i32;
// 	fn tcgetattr(__fd: i32, __termios_p: *mut termios) -> i32;
// 	fn tcsetattr(
// 		__fd: i32,
// 		__optional_actions: i32,
// 		__termios_p: *const termios,
// 	) -> i32;
// 	static mut g_ctrl_c: i32;
// }
// use gnu_readline_sys::{rl_on_new_line, rl_redisplay, rl_replace_line};
// use libc::write;
// use nix::sys::{signal::{SigAction, SigHandler, Signal::SIGINT}, termios::{LocalFlags, Termios}};

// use crate::{prelude::*, termios};

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct __sigset_t {
// 	pub __val: [libc::c_ulong; 16],
// }
// pub type sigset_t = __sigset_t;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union sigval {
// 	pub sival_int: i32,
// 	pub sival_ptr: *mut libc::c_void,
// }
// pub type __sigval_t = sigval;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct siginfo_t {
// 	pub si_signo: i32,
// 	pub si_errno: i32,
// 	pub si_code: i32,
// 	pub __pad0: i32,
// 	pub _sifields: C2RustUnnamed,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed {
// 	pub _pad: [i32; 28],
// 	pub _kill: C2RustUnnamed_8,
// 	pub _timer: C2RustUnnamed_7,
// 	pub _rt: C2RustUnnamed_6,
// 	pub _sigchld: C2RustUnnamed_5,
// 	pub _sigfault: C2RustUnnamed_2,
// 	pub _sigpoll: C2RustUnnamed_1,
// 	pub _sigsys: C2RustUnnamed_0,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_0 {
// 	pub _call_addr: *mut libc::c_void,
// 	pub _syscall: i32,
// 	pub _arch: libc::c_uint,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_1 {
// 	pub si_band: libc::c_long,
// 	pub si_fd: i32,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_2 {
// 	pub si_addr: *mut libc::c_void,
// 	pub si_addr_lsb: libc::c_short,
// 	pub _bounds: C2RustUnnamed_3,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed_3 {
// 	pub _addr_bnd: C2RustUnnamed_4,
// 	pub _pkey: __uint32_t,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_4 {
// 	pub _lower: *mut libc::c_void,
// 	pub _upper: *mut libc::c_void,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_5 {
// 	pub si_pid: __pid_t,
// 	pub si_uid: __uid_t,
// 	pub si_status: i32,
// 	pub si_utime: __clock_t,
// 	pub si_stime: __clock_t,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_6 {
// 	pub si_pid: __pid_t,
// 	pub si_uid: __uid_t,
// 	pub si_sigval: __sigval_t,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_7 {
// 	pub si_tid: i32,
// 	pub si_overrun: i32,
// 	pub si_sigval: __sigval_t,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct C2RustUnnamed_8 {
// 	pub si_pid: __pid_t,
// 	pub si_uid: __uid_t,
// }
// pub type __sighandler_t = Option<unsafe fn(i32) -> ()>;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct sigaction {
// 	pub __sigaction_handler: C2RustUnnamed_10,
// 	pub sa_mask: __sigset_t,
// 	pub sa_flags: i32,
// 	pub sa_restorer: Option<unsafe fn() -> ()>,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed_10 {
// 	pub sa_handler: __sighandler_t,
// 	pub sa_sigaction:
// 		Option<unsafe fn(i32, *mut siginfo_t, *mut libc::c_void) -> ()>,
// }
// #[unsafe(no_mangle)]
// pub unsafe fn check_signals(sigstruct: &mut Termios) {
// 	// tcgetattr(0 as i32, p_termios);
// 	// let mut sigstruct = nix::sys::termios::tcgetattr(io::stdin()).unwrap();
// 	sigstruct.local_flags &= LocalFlags::ECHOCTL;
// 	// (*p_termios).c_lflag &= !(0o1000 as i32) as libc::c_uint;
// 	// tcsetattr(0 as i32, 2 as i32, p_termios);
// 	nix::sys::termios::tcsetattr(
// 		std::io::stdin(),
// 		nix::sys::termios::SetArg::TCSAFLUSH,
// 		&sigstruct,
// 	)
// 	.unwrap();
// 	// ctrl-c-init
// 	{
// 		// let mut sig: sigaction = sigaction {
// 		// 	__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
// 		// 	sa_mask: __sigset_t { __val: [0; 16] },
// 		// 	sa_flags: 0,
// 		// 	sa_restorer: None,
// 		// };
// 		// sig.__sigaction_handler.sa_sigaction = Some(
// 		// 	ctrl_c_handler
// 		// 		as unsafe fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
// 		// );
// 		// sig.sa_flags = 0x10000000 as i32;
// 		// sigemptyset(&mut sig.sa_mask);
// 		// sigaction(2 as i32, &sig, std::ptr::null_mut::<sigaction>());
// 		// SigAction::new(handler, flags, mask)
// 		SigHandler::
// 		nix::sys::signal::sigaction(SIGINT, &SigAction { sigaction: ctrl_c_handler(sig, info, unused); })
// 	};
// 	// ctrl-bkslash-init
// 	{
// 		let mut sig: sigaction = sigaction {
// 			__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
// 			sa_mask: __sigset_t { __val: [0; 16] },
// 			sa_flags: 0,
// 			sa_restorer: None,
// 		};
// 		sig.__sigaction_handler.sa_handler = ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
// 			1 as i32 as libc::intptr_t,
// 		);
// 		sig.sa_flags = 0x10000000 as i32;
// 		sigemptyset(&mut sig.sa_mask);
// 		sigaction(3 as i32, &sig, std::ptr::null_mut::<sigaction>());
// 	};
// }

// unsafe fn ctrl_c_handler(
// 	mut sig: i32,
// 	mut _info: *mut siginfo_t,
// 	mut _unused: *mut libc::c_void,
// ) {
// 	if sig == 2 as i32 {
// 		g_ctrl_c = 1 as i32;
// 		write(
// 			0 as i32,
// 			b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
// 			1,
// 		);
// 		rl_on_new_line();
// 		rl_replace_line(b"\0" as *const u8 as *const libc::c_char, 0 as i32);
// 		rl_redisplay();
// 	}
// }

// #[unsafe(no_mangle)]
// pub unsafe fn check_signals_child(mut p_termios_child: *mut termios) {
// 	let mut attr: termios = termios {
// 		c_iflag: 0,
// 		c_oflag: 0,
// 		c_cflag: 0,
// 		c_lflag: 0,
// 		c_line: 0,
// 		c_cc: [0; 32],
// 		c_ispeed: 0,
// 		c_ospeed: 0,
// 	};
// 	tcgetattr(0 as i32, p_termios_child);
// 	tcgetattr(0 as i32, &mut attr);
// 	attr.c_lflag &= !(0o1000 as i32) as libc::c_uint;
// 	tcsetattr(1 as i32, 2 as i32, p_termios_child);
// 	ctrl_backslash_child();
// 	ctrl_c_child();
// }
// unsafe fn ctrl_c_child() {
// 	let mut ctrl_c: sigaction = sigaction {
// 		__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
// 		sa_mask: __sigset_t { __val: [0; 16] },
// 		sa_flags: 0,
// 		sa_restorer: None,
// 	};
// 	ctrl_c.__sigaction_handler.sa_handler = None;
// 	ctrl_c.sa_flags = 0x10000000 as i32;
// 	sigemptyset(&mut ctrl_c.sa_mask);
// 	sigaction(2 as i32, &ctrl_c, std::ptr::null_mut::<sigaction>());
// }
// unsafe fn ctrl_backslash_child() {
// 	let mut ctrl_slash: sigaction = sigaction {
// 		__sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
// 		sa_mask: __sigset_t { __val: [0; 16] },
// 		sa_flags: 0,
// 		sa_restorer: None,
// 	};
// 	ctrl_slash.__sigaction_handler.sa_handler = None;
// 	ctrl_slash.sa_flags = 0x10000000 as i32;
// 	sigemptyset(&mut ctrl_slash.sa_mask);
// 	sigaction(
// 		3 as i32,
// 		&ctrl_slash,
// 		std::ptr::null_mut::<sigaction>(),
// 	);
// }
