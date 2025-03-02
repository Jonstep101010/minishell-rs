# minishell-rs

A gradual rewrite of a [c2rust](https://github.com/immunant/c2rust) transpiled [codebase](https://gh.jschwabe.site/42_minishell).

Discover how C language programming constructs can be implemented in a more concise way.

## journey
### context

The original implementation used loads of custom glue that could have been replaced with libc functions (strtok, strcoll, scanf/sprintf, fprintf). 

This was fine as a school project and provided me with ample opportunities for refactoring in the rust version.

### process
1. transpile, then simplify some operations, mostly aligning types (e.g. [`libc::size_t`](https://docs.rs/libc/latest/libc/type.size_t.html) for [`u64`](https://doc.rust-lang.org/core/primitive.u64.html)/[`usize`](https://doc.rust-lang.org/core/primitive.usize.html)), removing non-needed casts and replacing [`.offset()`](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.offset) with [`.add()`](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.add)
2. multiple failed rewrites of core functionality caused by newly introduced logic bugs - mitigated by comprehensive [test cases](https://docs.rs/rstest/latest/rstest/attr.rstest.html#test-parametrized-cases)
3. issues with readability, naming - resolved by using more idiomatic constructs: [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html), [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html), [tuples](https://doc.rust-lang.org/std/primitive.tuple.html), [slices](https://doc.rust-lang.org/std/primitive.slice.html) instead of [references](https://doc.rust-lang.org/std/primitive.reference.html), [`impl`](https://doc.rust-lang.org/std/keyword.impl.html) for structs
4. remove duplicate or dead code replaced by std ([`format!`](https://doc.rust-lang.org/std/macro.format.html), [`vec![]`](https://doc.rust-lang.org/std/macro.vec.html), ...)

### goals
1. see where rust syntax and std containers can enable better readability/code structure
2. discover where [`CString`](https://doc.rust-lang.org/stable/std/ffi/struct.CString.html), [`CStr`](https://doc.rust-lang.org/stable/std/ffi/struct.CStr.html) and `&[u8]` ([u8](https://doc.rust-lang.org/stable/std/primitive.u8.html)) can find usage and reduce interactions with `char *`
3. replace `extern "C"`/[libc](https://docs.rs/libc/latest/libc/) usage with [nix](https://docs.rs/nix/latest/nix/) wrappers, enabling more idiomatic (and hopefully less error prone) usage of unix system functions.
4. provide me with an excuse to write rust code

### key takeaways
- rust is not c, interacting with raw pointers is more error prone due to its memory model and assumptions about memory carried over from c
- signal handling was easier to do in c, I ended up removing it as it caused weird bugs
- even though rust has [`Command`](https://doc.rust-lang.org/std/process/struct.Command.html), it was not used. I adapted my execution logic to be more idiomatic by using nix wrappers (the reason for most `unsafe` usage)

### refined strategy
1. document & simplify original logic
2. create adaptable tests, wrappers fitting both c-like and idiomatic rust outputs
3. apply TDD feedback loop while simplifying logic and reducing `unsafe`
