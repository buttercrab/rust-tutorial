# Hello, world!

## Description

Write a code that prints "Hello, world!"

## How to run this code

```sh
cargo run -p a-hello-world
```

## Examples

```sh
> cargo run -p a-hello-world
Hello, world!
```

## Tips

### `println!`

`prinln!` is a macro to print things to stdout.

The first argument of `println!` is a string literal: [rust playground][1]

```rust
fn main() {
    println!("123"); // prints "123"
}
```

We can use format string like: [rust playground][2]

```rust
fn main() {
    let a = 1;
    // implicit positioning
    println!("a = {}", a); // prints "a = 1"
}
```

If you want to position arguments: [rust playground][3]

```rust
fn main() {
    let a = 1;
    let b = 2;
    
    // explicit positioning
    // prints "a = 1, b = 2, a = 1"
    println!("a = {0}, b = {1}, a = {0}", a, b);

    // named argument
    // prints "a = 1, b = 2, a = 1"
    println!("a = {a}, b = {b}, a = {a}", a = a, b = b);
    
    // Since Rust 1.58.0, we can capture variables
    // prints "a = 1, b = 2, a = 1"
    println!("a = {a}, b = {b}, a = {a}");
}
```

We can debug print or format floats: [rust playground][4]

```rust
fn main() {
    let arr = [1.0, 1.1, 1.2];
    
    // prints "[1.0, 1.1, 1.2]"
    println!("{arr:?}");
    
    // pretty debug print
    // prints:
    // [
    //     1.0,
    //     1.1,
    //     1.2,
    // ]
    println!("{arr:#?}");
    
    // floating points
    // prints "1.100"
    println!("{:.3}", arr[1]);
}
```

### Cargo

Cargo is a package manager for Rust.

Here is some useful commands:

```sh
# this would make a directory with initial structure
cargo new <project-name>

# builds the package with debug mode
cargo build
# if binary is produced, run binary as:
./target/debug/<binary-name>

# builds the package with release mode (optimized)
cargo build --release
# if binary is produced, run binary as:
./target/release/<binary-name>

# Clippy is a tool for Rust linting
cargo clippy

# formats the Rust code
cargo fmt

# run all the tests
cargo test

# run the code in one command
cargo run
```

[1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1138c614ec1669299d3d742783aa2e23
[2]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f9e6a7dc3f85a2b7cb4fc184b68c5ff4
[3]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cd5017e89fb4e69f8dd2537b365cb05a
[4]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c32def0b5f9ea9f751610bf271e67c3b
