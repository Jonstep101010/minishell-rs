#[macro_export]
macro_rules! eprint_msh {
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            let stderr = std::io::stderr();
            let mut handle = stderr.lock();
            write!(handle, "minishell: ").unwrap();
            write!(handle, $($arg)*).unwrap();
            writeln!(handle).unwrap();
        }
    };
}
