# Binary Search Tree

## Description

Write a binary search tree with insert, search, remove feature with iterator support.

## How to compile this code

```sh
cargo build -p a-binary-tree
```

## How to format this code

```sh
cargo fmt -p a-binary-tree
```

## How to lint this code

```sh
cargo clippy -p a-binary-tree
```

## How to test this code

```sh
cargo test -p a-binary-tree
```

## Tips

### [`unsafe`][unsafe]

`unsafe` is a block of code that have more freedom from borrow checker.

Inside `unsafe` block, we can:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

[unsafe]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
