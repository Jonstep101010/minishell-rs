use crate::t_shell;

pub fn destroy_all_tokens(shell: &mut t_shell) {
	shell.token_len = None;
}
