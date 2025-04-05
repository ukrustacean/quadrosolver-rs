# quadrosolver-rs
A small 🚀blazing fast🚀 and ⚡memory safe⚡ CLI program for solving
quadratic equations, written with Rust. (Also my university assignment)

## Building and running

You will need the Rust Programming Language toolchain to run this program.
The easiest way to obtain one is using [rustup](https://rustup.rs/).
After you have `rustc` and `cargo` up and running, use the following
commands to clone, build and run the program:

```shell
$ git clone https://github.com/ukrustacean/quadrosolver-rs.git
$ cd ./quadrosolver-rs

# To build
$ cargo build

# To run
$ cargo run

# To run and pass file.txt as argument
$ cargo run -- file.txt
```

## Program modes

This program is capable of working in two modes: interactive and file.
In interactive mode, the program will prompt you to input the equation
arguments interactively through the CLI. In file mode it will read the
coefficients from a file you pass as an argument. Look up the
[Building and running](#building-and-running) section for instruction.

### File format

For program to be able to read the coefficients from the file properly,
it must obey the specified file format; the file can only contain a single
piece of text matching the following regular expression:

`^-?\d+(?:\.\d*)?\s-?\d+(?:\.\d*)?\s-?\d+(?:\.\d*)?\n$`

## Revert commit

Link to [commit](https://github.com/ukrustacean/quadrosolver-rs/commit/0ce0b3e4b2ed970fd06bbe074c1740192e6788cb).\
Its hash: `0ce0b3e4b2ed970fd06bbe074c1740192e6788cb`\
Ignore this, it's needed for my university assignment.

