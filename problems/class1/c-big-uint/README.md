# Big Unsigned Integer

## Description

Write a module that can handle big unsigned integer.
It contains basic operations, comparisons and string transformations.

## How to run this code

```sh
> cargo run -p c-big-uint
<number> <operator> <number>
<output>
```

## Examples

```sh
> cargo run -p c-big-uint
100 + 100
200
```

## How to test this code

```sh
cargo test -p c-big-uint
```

## Tips

### [`std::iter::Iterator`][iterator]

Iterator is a useful trait for iterating in containers. [rust playground][1]

```rust
fn main() {
    let v = vec![1, 2, 3];

    v.iter().for_each(|x| println!("{x}"));
    let a = v.iter().map(|x| x * 2).collect::<Vec<_>>();
    let b = a.iter().fold(0, |x, i| x + i);
    println!("b = {b}");
}
```

There are a lot of useful functions that we can use. Check out the document for more information.


### Operator overloading

In Rust, we can overload operators by implementing traits in [`std::ops`][ops].
However you cannot define a new operator.
[rust playground][2]

```rust
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
```

### [`std::result::Result`][result]

It is a enum for the result of anything that can contain error.
If the value is `Ok(x)`, then the result is ok.
If the value is `Err(x)`, then there was an error when producing result.

The question mark operator (`?`) can be used for control flow breaks.
[rust playground][3]

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

[iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6570797f61c1873630506b649544a6ab
[ops]: https://doc.rust-lang.org/std/ops/index.html
[result]: https://doc.rust-lang.org/std/result/
[2]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3f9dd79b3c8d53ace94f252fad2af057
[3]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f9af30f511bdbe342119574c615e494e