# Fibonacci number with loops

## Description

Write functions that returns fibonacci number.
Write each function with different ways using loop and recursion.

## How to run this code

```sh
cargo run -p b-fib-loop -- <input>
```

## Examples

```sh
> cargo run -p b-fib-loop -- 10
55
```

## How to test this code

```sh
cargo test -p b-fib-loop
```

## Tips

### `std::vec::Vec`

`Vec` is a dynamic array. ([official document][vec document]) We can use same syntax as array to access, change the value like: [rust playground][1]

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    println!("{}", v[0]); // prints "1"

    v[1] = 1;
    println!("{}", v[1]); // prints "1"

    v.push(4);
    println!("{v:?}"); // prints "[1, 1, 3, 4]"
}
```

### function in function

In rust, we can use function in function.
The variables would not be captured inside like: [rust playground][2]

```rust
fn outer() -> usize {
    let a = 10;
    
    fn inner() -> usize {
        a + 10 // compile error
    }
    
    inner()
}
```

[vec document]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3f325f314b4c037b49ab203199361037
[2]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=20c20619453ddd29e6fac68cebb61c76