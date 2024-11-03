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
	fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
	fn rl_redisplay();
	fn rl_on_new_line() -> libc::c_int;
	fn rl_replace_line(_: *const libc::c_char, _: libc::c_int);
	static mut g_ctrl_c: libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub struct termios {
	pub c_iflag: tcflag_t,
	pub c_oflag: tcflag_t,
	pub c_cflag: tcflag_t,
	pub c_lflag: tcflag_t,
	pub c_line: cc_t,
	pub c_cc: [cc_t; 32],
	pub c_ispeed: speed_t,
	pub c_ospeed: speed_t,
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
	pub __sigaction_handler: C2RustUnnamed_9,
	pub sa_mask: __sigset_t,
	pub sa_flags: libc::c_int,
	pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
	pub sa_handler: __sighandler_t,
	pub sa_sigaction:
		Option<unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ()>,
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
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
		__sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
		sa_mask: __sigset_t { __val: [0; 16] },
		sa_flags: 0,
		sa_restorer: None,
	};
	sig.__sigaction_handler.sa_handler = ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(
		1 as libc::c_int as libc::intptr_t,
	);
	sig.sa_flags = 0x10000000 as libc::c_int;
	sigemptyset(&mut sig.sa_mask);
	sigaction(3 as libc::c_int, &mut sig, 0 as *mut sigaction);
}
unsafe extern "C" fn ctrl_c_init() {
	let mut sig: sigaction = sigaction {
		__sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
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
	sigaction(2 as libc::c_int, &mut sig, 0 as *mut sigaction);
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
			1 as libc::c_int as size_t,
		);
		rl_on_new_line();
		rl_replace_line(b"\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
		rl_redisplay();
	}
}
