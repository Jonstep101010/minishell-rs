# minishell

A gradual rewrite of a [c2rust](https://github.com/immunant/c2rust) transpiled [project](https://gh.jschwabe.site/42_minishell).

This was a personal project to see how C systems programming constructs/syscalls can be implemented in a more concise way.

## journey

FWIW, the original implementation used loads of custom glue that could have been replaced with standard library functions. This was fine as a school project and provided me with ample opportunities for refactoring in the rust version.

1. transpile, then simplify some operations, mostly aligning types and replacing `.offset()` with `.add()`

## 

### goals
1. see where rust syntax and std containers can enable better readability/code structure
2. discover where [`CString`](https://doc.rust-lang.org/stable/std/ffi/struct.CString.html), [`CStr`](https://doc.rust-lang.org/stable/std/ffi/struct.CStr.html) and `&[u8]` ([u8](https://doc.rust-lang.org/stable/std/primitive.u8.html)) can find usage and reduce interactions with `char *`
3. replace `extern "C"`/[libc](https://docs.rs/libc/latest/libc/) usage with [nix](https://docs.rs/nix/latest/nix/) wrappers, enabling more idiomatic (and hopefully less error prone) usage of unix system functions.
4. provide me with an excuse to write rust code

### key takeaways
- rust is not c, interacting with raw pointers is more error prone due to its memory model and assumptions about memory carried over from c
- signal handling was easier to do in c (I ended up just scrapping it in the end)
- even though rust has [`Command`](https://doc.rust-lang.org/std/process/struct.Command.html), I adapted my execution logic anyways (the reason for most `unsafe` usage)

### refined strategy
1. document & simplify original logic
2. create adaptable tests, wrappers fitting both c-like and idiomatic rust outputs
3. apply TDD feedback loop while simplifying logic and reducing `unsafe`
