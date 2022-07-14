//! Fibonacci number calculator
//!
//! ## How to run this code
//!
//! ```sh
//! cargo run -p b-fibonacci -- <input>
//! ```
//!
//! ## Examples
//!
//! ```sh
//! > cargo run -p b-fibonacci -- 10
//! by loop: 55
//! by recursion: 55
//! ```
//!
//! ## How to test this code
//!
//! ```sh
//! cargo test -p b-fibonacci
//! ```

use std::env;

/// Returns a `n`-th [fibonacci number][fibonacci]
///
/// ## Example
///
/// ```
/// assert_eq!(fib_loop(0), 0);
/// ```
///
/// [fibonacci]: https://en.wikipedia.org/wiki/Fibonacci_number
fn fib_loop(n: usize) -> usize {
    todo!("n = {n}")
}

/// Returns a `n`-th [fibonacci number][fibonacci]
///
/// ## Example
///
/// ```
/// assert_eq!(fib_recur(0), 0);
/// ```
///
/// [fibonacci]: https://en.wikipedia.org/wiki/Fibonacci_number
fn fib_recur(n: usize) -> usize {
    /// Inner recursive function for `O(n)` complexity
    fn inner(arr: &mut Vec<usize>, n: usize) -> usize {
        todo!("arr = {arr:?}, n = {n}")
    }

    // this would create `Vec` of size `n` filled with `0`.
    let mut arr = vec![0; n + 1];
    inner(&mut arr, n)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let n = args[1].parse::<usize>().unwrap();
    println!("by loop: {}", fib_loop(n));
    println!("by recursion: {}", fib_recur(n));
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! make_test_small {
        ($test_name:ident, $fn_name:ident) => {
            #[test]
            fn $test_name() {
                assert_eq!($fn_name(0), 0);
                assert_eq!($fn_name(1), 1);
                assert_eq!($fn_name(2), 1);
                assert_eq!($fn_name(3), 2);
                assert_eq!($fn_name(4), 3);
                assert_eq!($fn_name(5), 5);
                assert_eq!($fn_name(6), 8);
                assert_eq!($fn_name(7), 13);
            }
        };
    }

    make_test_small!(test_loop_small, fib_loop);
    make_test_small!(test_recur_small, fib_recur);

    macro_rules! make_test_large {
        ($test_name:ident, $fn_name:ident) => {
            #[test]
            fn $test_name() {
                assert_eq!($fn_name(50), 12_586_269_025);
                assert_eq!($fn_name(70), 190_392_490_709_135);
            }
        };
    }

    make_test_large!(test_loop_large, fib_loop);
    make_test_large!(test_recur_large, fib_recur);
}
