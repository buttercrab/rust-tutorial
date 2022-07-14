# Default Argument

## Description

Write a macro for handling default argument.
The macro will generate another macro that can be used in default argument.

## Example

```rust
// this would generate macro `f`
default_args! {
    fn f(a: u32 = 10) -> u32 {
        a + 10
    }
}

fn main() {
    // that is used in here
    assert_eq!(f!(), 20);
    assert_eq!(f!(20), 30);
}
```

## Tips

- [Macro Reference](https://doc.rust-lang.org/reference/macros.html)
- [Rust Book - Macro](https://doc.rust-lang.org/book/ch19-06-macros.html)