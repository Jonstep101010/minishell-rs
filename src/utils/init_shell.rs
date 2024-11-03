use ::libc;
extern "C" {
	fn free(_: *mut libc::c_void);
	fn exit(_: libc::c_int) -> !;
	fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
	fn ft_calloc(nitems: size_t, size: size_t) -> *mut libc::c_void;
	fn ft_strdup(s: *const libc::c_char) -> *mut libc::c_char;
	fn append_str_arr(
		arr: *const *mut libc::c_char,
		s: *const libc::c_char,
	) -> *mut *mut libc::c_char;
	fn arr_free(arr: *mut *mut libc::c_char);
	fn append_str_arr_free(
		arr: *mut *mut libc::c_char,
		s: *mut libc::c_char,
	) -> *mut *mut libc::c_char;
	fn get_env(env: *const *mut libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
	fn free_second_join(s1: *const libc::c_char, s2: *mut libc::c_char) -> *mut libc::c_char;
	fn free_null(p: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_token {
	pub cmd_args: *mut t_arg,
	pub has_redir: bool,
	pub split_pipes: *mut libc::c_char,
	pub tmp_arr: *mut *mut libc::c_char,
	pub bin: *mut libc::c_char,
	pub cmd_func: Option<unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int>,
}
pub type t_token = s_token;
pub type t_shell = s_shell;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_shell {
	pub exit_status: uint8_t,
	pub env: *mut *mut libc::c_char,
	pub token: *mut t_token,
	pub token_len: size_t,
	pub p_termios: termios,
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
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type t_arg = s_arg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_arg {
	pub elem: *mut libc::c_char,
	pub type_0: e_arg,
	pub redir: e_redir,
}
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
unsafe extern "C" fn init_env(mut envp: *const *mut libc::c_char) -> *mut *mut libc::c_char {
	let mut env: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
	let mut pwd: *mut libc::c_char = 0 as *mut libc::c_char;
	let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
	env = append_str_arr(envp, b"?=0\0" as *const u8 as *const libc::c_char);
	pwd = get_env(env, b"PWD\0" as *const u8 as *const libc::c_char);
	if pwd.is_null() && !env.is_null() {
		pwd = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
		if pwd.is_null() {
			arr_free(env);
			return 0 as *mut libc::c_void as *mut *mut libc::c_char;
		}
		env = append_str_arr_free(
			append_str_arr_free(
				env,
				free_second_join(b"PWD=\0" as *const u8 as *const libc::c_char, pwd),
			),
			ft_strdup(b"OLDPWD=''\0" as *const u8 as *const libc::c_char),
		);
	} else {
		free_null(&mut pwd as *mut *mut libc::c_char as *mut libc::c_void);
	}
	path = get_env(env, b"PATH\0" as *const u8 as *const libc::c_char);
	if path.is_null() {
		env = append_str_arr_free(
			env,
			ft_strdup(b"PATH=/bin:/usr/bin:/sbin/:/usr/sbin\0" as *const u8 as *const libc::c_char),
		);
	} else {
		free_null(&mut path as *mut *mut libc::c_char as *mut libc::c_void);
	}
	return env;
}
#[no_mangle]
pub unsafe extern "C" fn init_shell(mut envp: *const *mut libc::c_char) -> *mut t_shell {
	let mut shell: *mut t_shell = 0 as *mut t_shell;
	shell = ft_calloc(
		1 as libc::c_int as size_t,
		::core::mem::size_of::<t_shell>() as libc::c_ulong,
	) as *mut t_shell;
	if shell.is_null() {
		exit(1 as libc::c_int);
	}
	(*shell).p_termios = {
		let mut init = termios {
			c_iflag: 0 as libc::c_int as tcflag_t,
			c_oflag: 0,
			c_cflag: 0,
			c_lflag: 0,
			c_line: 0,
			c_cc: [0; 32],
			c_ispeed: 0,
			c_ospeed: 0,
		};
		init
	};
	(*shell).env = init_env(envp);
	if ((*shell).env).is_null() {
		free(shell as *mut libc::c_void);
		return 0 as *mut t_shell;
	}
	return shell;
}
