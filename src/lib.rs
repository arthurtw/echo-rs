//! Macro `echo!` and `echon!` print values separated by spaces without the
//! need to specify `"{}"` format strings, similar to Linux `echo` and
//! `echo -n` commands.
//!
//! To use the macro, you'll need to include the following declarations
//! at the top level of your crate:
//!
//! ```ignore
//! #![feature(phase)]
//! #[phase(plugin)] extern crate echo;
//! ```
//!
//! Then you can invoke it as follows:
//!
//! ```ignore
//! let a = 0u;
//! let b = vec![2i, 4, 6];
//! // 0 [2, 4, 6] true
//! echo!(a, b, true);
//! // 0 (without newline)
//! echon!(a);
//! ```
#![feature(macro_rules)]

#[macro_export]
/// Print space-separated values with newline
macro_rules! echo {
    ($($arg:tt)*) => ({
        echon!($($arg)*)
        println!("")
    });
}

#[macro_export]
/// Print space-separated values without newline
macro_rules! echon {
    ($head:expr $(, $tail:expr)*) => ({
        print!("{}", $head);
        $(print!(" {}", $tail);)*
    });
}
