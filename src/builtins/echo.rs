use crate::prelude::*;

use libft_rs::ft_strncmp::ft_strncmp;
use libutils_rs::src::array::arr_free::arr_free;

unsafe fn is_n_arg(mut arg: *const c_char) -> bool {
	if *arg as c_int == '-' as i32 {
		arg = arg.add(1);
		while *arg as c_int == 'n' as i32 {
			arg = arg.add(1);
			if *arg as c_int == '\0' as i32 {
				return true;
			}
		}
	}
	false
}

unsafe fn echo_default(cmd_args: *const *const c_char) {
	let mut i = 0;
	while !(*cmd_args.add(i)).is_null() && is_n_arg(*cmd_args.add(i)) {
		i += 1;
	}
	let mut flag = match i {
		0 => 2,
		_ => 1,
	};
	let n_pos = i;
	while !(*cmd_args.add(i)).is_null() {
		if flag == 1 as c_int
			&& ft_strncmp(*cmd_args.add(i), c"-n".as_ptr(), 2u64) == 0 as c_int
			&& *(*cmd_args.add(i)).add(2) as c_int != '\0' as i32
			&& i != n_pos + 1
		{
			flag = 0;
		}
		print!(
			"{}",
			std::ffi::CStr::from_ptr(*cmd_args.add(i))
				.to_str()
				.expect("CStr::from_bytes_with_nul failed")
		);
		if !(*cmd_args.add(i + 1)).is_null() {
			print!(" ");
		}
		i += 1;
	}
	if flag != 1 as c_int {
		println!();
	}
}

pub unsafe fn echo(args: *mut *const c_char) -> i32 {
	if !(*args.add(1)).is_null() {
		echo_default(&*args.add(1));
	} else {
		println!();
	}
	arr_free(args as *mut *mut c_char);
	0
}
