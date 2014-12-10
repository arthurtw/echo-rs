# echo

Rust macro `echo!` and `echon!` as a shorthand for `println!("{}"...)` and `print!("{}"...)`.

## Usage

To use this library, add the following to your `Cargo.toml` file:

```
[dependencies]
echo = "*"
```

You can then use macro `echo!` and `echon!` to print space-separated values
with or without newline, similar to Linux `echo` and `echo -n` commands.

```rust
#![feature(phase)]
#[phase(plugin)] extern crate echo;

fn main() {
    let a = 0u;
    let b = vec![2i, 4, 6];
    // 0 [2, 4, 6] true
    echo!(a, b, true);
    // 0 (without newline)
    echon!(a);
}
```

## License

MIT

