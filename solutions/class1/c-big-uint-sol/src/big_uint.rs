use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops;
use std::ops::ControlFlow;
use std::str::FromStr;

/// Big unsigned integer module
///
/// ## Example
///
/// ```
/// # use crate::big_int::BigUInt;
/// # fn main() {
/// let a = BigUInt::one();
/// let b = BigUInt::from(1_000_000usize);
/// let c = BigUInt::from_str("1000000000000000000000");
/// # }
/// ```
///
/// ## Implementation
///
/// The big number is implemented by vector of `u64`.
/// Each element can have the value of range `0` to `u64::MAX`.
/// The value of the number is `u64::MAX + 1` when the vector is `[0, 1]`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BigUInt {
    inner: Vec<u64>,
}

impl BigUInt {
    pub const fn zero() -> BigUInt {
        BigUInt { inner: Vec::new() }
    }

    pub fn one() -> BigUInt {
        BigUInt { inner: vec![1] }
    }

    /// Shift left once; same as *2 of the value
    fn shl_once(&mut self) {
        let mut carry = false;

        for i in self.inner.iter_mut() {
            *i = i.rotate_left(1);
            let c = *i & 1;
            *i = (*i ^ c) | carry as u64;
            carry = c != 0;
        }

        if carry {
            self.inner.push(1);
        }
    }

    /// Shift right once; same as /2 of the value
    fn shr_once(&mut self) {
        let mut carry = false;

        for i in self.inner.iter_mut().rev() {
            let c = *i & 1 != 0;
            *i >>= 1;
            *i |= (carry as u64) << 63;
            carry = c;
        }

        if let Some(&0) = self.inner.last() {
            self.inner.pop();
        }
    }
}

/// Error for parsing `BigUInt`.
#[derive(Debug, Copy, Clone)]
pub enum ParseBigUIntError {
    Empty,
    NotStartingWithDigit,
    LeadingZero,
    InvalidCharacter,
}

impl Display for ParseBigUIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseBigUIntError::Empty => write!(f, "empty string"),
            ParseBigUIntError::NotStartingWithDigit => {
                write!(f, "number does not start with digit")
            }
            ParseBigUIntError::LeadingZero => write!(f, "number contains leading zero"),
            ParseBigUIntError::InvalidCharacter => write!(f, "number contains invalid character"),
        }
    }
}

impl FromStr for BigUInt {
    type Err = ParseBigUIntError;

    /// String to big integer. Input can contain `,` and `_` and will be ignored.
    /// If the string is invalid, it will return an error with description.
    ///
    /// ## Examples
    ///
    /// ```
    /// let c = BigUInt::from_str("1,000,000,000,000,000,000,000,000");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first) = s.chars().next() {
            if first == '0' {
                if s.len() == 1 {
                    Ok(BigUInt::zero())
                } else {
                    Err(ParseBigUIntError::LeadingZero)
                }
            } else if !first.is_ascii_digit() {
                Err(ParseBigUIntError::NotStartingWithDigit)
            } else {
                s.chars().try_fold(BigUInt::zero(), |a, i| match i {
                    '_' | ',' => Ok(a),
                    '0'..='9' => {
                        let a = &a * &BigUInt::from(10u64);
                        Ok(&a + &BigUInt::from(i as u8 - b'0'))
                    }
                    _ => Err(ParseBigUIntError::InvalidCharacter),
                })
            }
        } else {
            Err(ParseBigUIntError::Empty)
        }
    }
}

impl<'a, T> ops::Add<T> for &BigUInt
where
    T: Into<&'a BigUInt>,
{
    type Output = BigUInt;

    /// Add operator between two numbers.
    ///
    /// ## Examples
    ///
    /// ```
    /// let a = BigUInt::from(100u64);
    /// let b = BigUInt::from(150u64);
    ///
    /// assert_eq!(&a + &b, BigUInt::from(250u64));
    /// ```
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();

        if self.inner.len() < rhs.inner.len() {
            rhs + self
        } else {
            let mut inner = self.inner.clone();
            let mut carry = false;

            for (i, x) in inner.iter_mut().enumerate() {
                (*x, carry) = x.overflowing_add(carry as u64);
                if let Some(a) = rhs.inner.get(i) {
                    let (b, c) = x.overflowing_add(*a);
                    *x = b;
                    carry |= c;
                } else if !carry {
                    break;
                }
            }

            if carry {
                inner.push(1);
            }

            BigUInt { inner }
        }
    }
}

impl<'a, T> ops::Sub<T> for &BigUInt
where
    T: Into<&'a BigUInt>,
{
    type Output = BigUInt;

    /// Sub operator between two numbers.
    /// If the rhs is bigger than lhs, it should return `0`.
    ///
    /// ## Examples
    ///
    /// ```
    /// let a = BigUInt::from(150u64);
    /// let b = BigUInt::from(100u64);
    ///
    /// assert_eq!(&a - &b, BigUInt::from(50u64));
    /// ```
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();

        if self <= rhs {
            BigUInt::zero()
        } else {
            let mut inner = self.inner.clone();
            let mut borrow = false;

            for (i, x) in inner.iter_mut().enumerate() {
                (*x, borrow) = x.overflowing_sub(borrow as u64);
                if let Some(a) = rhs.inner.get(i) {
                    let (b, c) = x.overflowing_sub(*a);
                    *x = b;
                    borrow |= c;
                } else if !borrow {
                    break;
                }
            }

            while let Some(&0) = inner.last() {
                inner.pop();
            }

            BigUInt { inner }
        }
    }
}

impl<'a, T> ops::Mul<T> for &BigUInt
where
    T: Into<&'a BigUInt>,
{
    type Output = BigUInt;

    /// Mul operator between two numbers.
    ///
    /// ## Examples
    ///
    /// ```
    /// let a = BigUInt::from(150u64);
    /// let b = BigUInt::from(100u64);
    ///
    /// assert_eq!(&a * &b, BigUInt::from(15000u64));
    /// ```
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let mut ans = BigUInt::zero();

        for (i, x) in rhs.inner.iter().enumerate() {
            let mut inner = self.inner.clone();
            let mut carry = 0u64;

            for j in inner.iter_mut() {
                let mul = *j as u128 * *x as u128 + carry as u128;
                *j = mul as u64;
                carry = (mul >> 64) as u64;
            }

            if carry > 0 {
                inner.push(carry);
            }

            ans = &ans
                + &BigUInt {
                    inner: std::iter::repeat(0)
                        .take(i)
                        .chain(inner.into_iter())
                        .collect(),
                };
        }

        ans
    }
}

/// This function returns division and remainder of lhs and rhs.
fn div_rem(lhs: &BigUInt, rhs: &BigUInt) -> (BigUInt, BigUInt) {
    if rhs.inner.is_empty() {
        panic!("division by zero");
    } else if lhs < rhs {
        (BigUInt::zero(), lhs.clone())
    } else {
        let mut lhs = lhs.clone();
        let mut rhs = rhs.clone();
        let mut shift = BigUInt::one();
        let mut ans = BigUInt::zero();

        while lhs >= rhs {
            rhs.shl_once();
            shift.shl_once();
        }

        while !lhs.inner.is_empty() && !shift.inner.is_empty() {
            if lhs >= rhs {
                lhs = &lhs - &rhs;
                ans = &ans + &shift;
            }

            rhs.shr_once();
            shift.shr_once();
        }

        (ans, lhs)
    }
}

impl<'a, T> ops::Div<T> for &BigUInt
where
    T: Into<&'a BigUInt>,
{
    type Output = BigUInt;

    /// Div operator between two numbers.
    /// If rhs is `0` it should panic as `division by zero`.
    ///
    /// ## Examples
    ///
    /// ```
    /// let a = BigUInt::from(150u64);
    /// let b = BigUInt::from(11u64);
    ///
    /// assert_eq!(&a / &b, BigUInt::from(11u64));
    /// ```
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        div_rem(self, rhs).0
    }
}

impl<'a, T> ops::Rem<T> for &BigUInt
where
    T: Into<&'a BigUInt>,
{
    type Output = BigUInt;

    /// Rem operator between two numbers.
    /// If rhs is `0` it should panic as `division by zero`.
    ///
    /// ## Examples
    ///
    /// ```
    /// let a = BigUInt::from(150u64);
    /// let b = BigUInt::from(11u64);
    ///
    /// assert_eq!(&a % &b, BigUInt::from(7u64));
    /// ```
    fn rem(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        div_rem(self, rhs).1
    }
}

impl PartialOrd for BigUInt {
    /// Comparison operator between two numbers.
    ///
    /// ## Example
    ///
    /// ```
    /// let a = BigUInt::from(150u64);
    /// let b = BigUInt::from(100u64);
    ///
    /// assert!(a > b);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.inner.len().cmp(&other.inner.len()) {
            Ordering::Equal => match self
                .inner
                .iter()
                .rev()
                .zip(other.inner.iter().rev())
                .try_for_each(|(x, y)| match x.cmp(y) {
                    Ordering::Equal => ControlFlow::Continue(()),
                    x => ControlFlow::Break(x),
                }) {
                ControlFlow::Continue(_) => Some(Ordering::Equal),
                ControlFlow::Break(x) => Some(x),
            },
            x => Some(x),
        }
    }
}

impl Display for BigUInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut t = self.clone();
        let mut v = Vec::new();

        if self.inner.is_empty() {
            write!(f, "0")
        } else {
            let str_mod = BigUInt::from(1_000_000_000_000_000_000u64);

            while !t.inner.is_empty() {
                let a = &t % &str_mod;
                v.push(if let Some(x) = a.inner.first() { *x } else { 0 });
                t = &t / &str_mod;
            }

            v.into_iter()
                .rev()
                .try_fold(true, |first, t| {
                    if first {
                        write!(f, "{t}")?;
                    } else {
                        write!(f, "{t:018}")?;
                    }
                    Ok(false)
                })
                .map(|_| ())
        }
    }
}

macro_rules! big_uint_from_impl {
    ($i:ty) => {
        impl From<$i> for BigUInt {
            fn from(x: $i) -> Self {
                BigUInt {
                    inner: vec![x as u64],
                }
            }
        }
    };
}

big_uint_from_impl!(u8);
big_uint_from_impl!(u16);
big_uint_from_impl!(u32);
big_uint_from_impl!(u64);
big_uint_from_impl!(usize);

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::BigUInt;

    #[test]
    fn string_test() {
        assert_eq!(BigUInt::zero().to_string(), "0");
        assert_eq!(BigUInt::one().to_string(), "1");

        let a = BigUInt::from_str("1,000,000,000,000,000,000,000,000").unwrap();
        assert_eq!(a.to_string(), "1000000000000000000000000");

        let b = BigUInt::from_str("76345621812716237612783617236178236712378123").unwrap();
        assert_eq!(
            b.to_string(),
            "76345621812716237612783617236178236712378123"
        );
    }

    #[test]
    fn add_test() {
        fn case(a: &str, b: &str, c: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            let c = BigUInt::from_str(c).unwrap();
            assert_eq!(&a + &b, c);
        }

        case(
            "1,000,000,000,000,000,000,000,000",
            "1,000,000,000,000,000,000,000,000",
            "2,000,000,000,000,000,000,000,000",
        );
        case(
            "1234567890987654321",
            "9876543210123456789",
            "11111111101111111110",
        );
        case(
            "1,000,000,000,000,000,000,000,000",
            "1",
            "1,000,000,000,000,000,000,000,001",
        );
    }

    #[test]
    fn sub_test() {
        fn case(a: &str, b: &str, c: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            let c = BigUInt::from_str(c).unwrap();
            assert_eq!(&a - &b, c);
        }

        case(
            "1,000,000,000,000,000,000,000,000",
            "1,000,000,000,000,000,000,000,000",
            "0",
        );
        case(
            "9876543210123456789",
            "1234567890987654321",
            "8641975319135802468",
        );
        case("1", "2", "0");
    }

    #[test]
    fn mul_test() {
        fn case(a: &str, b: &str, c: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            let c = BigUInt::from_str(c).unwrap();
            assert_eq!(&a * &b, c);
        }

        case(
            "1,000,000,000,000,000,000,000,000",
            "1,000,000,000,000,000,000,000,000",
            "1,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000",
        );
        case(
            "9876543210123456789",
            "1234567890987654321",
            "12193263121170553265523548251112635269",
        );
        case("1", "0", "0");
    }

    #[test]
    fn div_test() {
        fn case(a: &str, b: &str, c: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            let c = BigUInt::from_str(c).unwrap();
            assert_eq!(&a / &b, c);
        }

        case(
            "1,000,000,000,000,000,000,000,000",
            "1,000,000,000,000,000,000,000,000",
            "1",
        );
        case("9876543210123456789", "123456789", "80000000730");
        case("123456789", "9876543210123456789", "0");
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn div_by_zero_test() {
        fn case(a: &str, b: &str, c: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            let c = BigUInt::from_str(c).unwrap();
            assert_eq!(&a / &b, c);
        }

        case("1", "0", "0");
    }

    #[test]
    fn rem_test() {
        fn case(a: &str, b: &str, c: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            let c = BigUInt::from_str(c).unwrap();
            assert_eq!(&a % &b, c);
        }

        case(
            "1,000,000,000,000,000,000,000,000",
            "1,000,000,000,000,000,000,000,000",
            "0",
        );
        case("9876543210123456789", "123456789", "819");
        case("123456789", "9876543210123456789", "123456789")
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn rem_by_zero_test() {
        fn case(a: &str, b: &str, c: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            let c = BigUInt::from_str(c).unwrap();
            assert_eq!(&a % &b, c);
        }

        case("1", "0", "0");
    }

    #[test]
    fn cmp_test() {
        fn case(a: &str, b: &str) {
            let a = BigUInt::from_str(a).unwrap();
            let b = BigUInt::from_str(b).unwrap();
            assert!(a < b);
        }

        case(
            "1,000,000,000,000,000,000,000,000",
            "1,100,000,000,000,000,000,000,000",
        );
        case(
            "100,000,000,000,000,000,000,000",
            "1,000,000,000,000,000,000,000,000",
        );
        case(
            "999_999_999_999_999_999_999_999",
            "1,000,000,000,000,000,000,000,000",
        );
    }
}
