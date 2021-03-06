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

#[macro_export]
macro_rules! default_args {
    (
        fn $fn_name:ident ( $arg_name:ident : $arg_type:ty = $value:expr ) $( -> $ret_type:ty)? $body:block
    ) => {
        macro_rules! $fn_name {
            () => {
                $fn_name ( $value )
            };
            ( $v:expr ) => {
                $fn_name ( $v )
            };
        }

        fn $fn_name ( $arg_name : $arg_type ) $( -> $ret_type )? $body
    };
}

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
