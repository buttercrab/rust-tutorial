//! Default Argument Macro
//!
//! It can handle one parameter function with default value.
//!
//! ## Example
//!
//! ```
//! default_args! {
//!     fn f(a: u32 = 10) -> u32 {
//!         a + 10
//!     }
//! }
//!
//! # fn main() {
//! assert_eq!(f!(), 20);
//! assert_eq!(f!(20), 30);
//! # }
//! ```

// make default_args macro
todo!();

#[cfg(test)]
mod test {
    default_args! {
        fn f(a: u32 = 10) -> u32 {
            a + 10
        }
    }

    #[test]
    fn simple_test() {
        assert_eq!(f!(), 20);
        assert_eq!(f!(20), 30);
    }
}
