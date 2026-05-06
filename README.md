# bsd-getopt

A small BSD-style getopt implementation in Rust.

## Features

- Short options (`-a`, `-b value`)
- Grouped options (`-abc`)
- Option arguments (`-ovalue` or `-o value`)
- `--` terminator

## Example

```rust
use bsd_getopt::Getopt;

let args = vec![
    "prog".to_string(),
    "-a".to_string(),
    "-bfoo".to_string(),
];

let mut g = Getopt::new("ab:", args);

while let Some(opt) = g.next() {
    println!("opt: {}, arg: {:?}", opt, g.optarg);
}