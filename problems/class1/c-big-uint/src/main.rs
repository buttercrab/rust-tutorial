//! Big Integer
//!
//! ## How to run this code
//!
//! ```sh
//! > cargo run -p c-big-uint
//! <number> <operator> <number>
//! <output>
//! ```
//!
//! ## Examples
//!
//! ```sh
//! > cargo run -p c-big-uint
//! 100 + 100
//! 200
//! ```
//!
//! ## How to test this code
//!
//! ```sh
//! cargo test -p c-big-uint
//! ```

use std::io;
use std::io::BufRead;
use std::str::FromStr;

use crate::big_uint::BigUInt;

mod big_uint;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let input = input.split(' ').collect::<Vec<_>>();

    let a = BigUInt::from_str(input[0]).unwrap();
    let b = BigUInt::from_str(input[2]).unwrap();

    let c = match input[1] {
        "+" => (&a + &b).to_string(),
        "-" => (&a - &b).to_string(),
        "*" => (&a * &b).to_string(),
        "/" => (&a / &b).to_string(),
        "%" => (&a % &b).to_string(),
        "==" => (a == b).to_string(),
        "!=" => (a != b).to_string(),
        "<" => (a < b).to_string(),
        ">" => (a > b).to_string(),
        "<=" => (a <= b).to_string(),
        ">=" => (a >= b).to_string(),
        _ => return,
    };

    println!("{c}");
}
