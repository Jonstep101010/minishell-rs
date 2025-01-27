use crate::prelude::*;
use nix::errno::Errno;

pub fn exec_bin(shell_env: &Env, slice_args: &[CString]) -> ! {
	// @follow-up make this a result
	let (access_status, path_prefixed_bin) =
		super::bin_path::get_path_prefixed(shell_env, &slice_args[0]);
	if let Some(path_exec_bin) = path_prefixed_bin {
		let slice_env = shell_env.to_cstring_vec();
		if nix::unistd::execve(&path_exec_bin, slice_args, slice_env.as_slice()).is_err() {
			todo!("handle execve_fail");
		}
	} else if access_status == 126 && !matches!(slice_args[0].as_bytes()[0], b'/' | b'~') {
		eprint_msh!("{}: {}", slice_args[0].to_str().unwrap(), Errno::last());
	} else if access_status == 127 {
		eprint_msh!("{:?}: command not found", slice_args);
	}
	std::process::exit(access_status.into());
}
