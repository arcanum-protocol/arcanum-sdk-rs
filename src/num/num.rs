use core::{
    fmt::{Debug, Formatter},
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign},
};
use std::ops::{DivAssign, MulAssign};

use borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::uint256::{mul_div256, U256};
use super::utils::{
    cast_decimals,
    strings::{cast_float_to_integer, cast_integer_to_float},
};

/// Fixed point number representation.
#[derive(
    BorshSerialize, BorshDeserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default,
)]
pub struct Num {
    value: U256,
}

impl Num {
    pub const MAX: Self = Self { value: U256::MAX };

    pub const MIN: Self = Self {
        value: U256([0, 0, 0, 0]),
    };

    pub const ZERO: Self = Self {
        value: U256([0, 0, 0, 0]),
    };

    pub const ONE: Self = Self {
        value: U256([1, 0, 0, 0]),
    };

    pub const DECIMALS: u8 = 24;

    // 10^24
    pub const DENOMINATOR: U256 = U256([2003764205206896640, 54210, 0, 0]);

    /// Creates a new `Decimal` from raw number and decimals.
    pub fn with_decimals(value: U256, decimals: u8) -> Num {
        Num {
            value: cast_decimals(value, decimals, Self::DECIMALS),
        }
    }

    /// Scale number to the given decimals.
    pub fn scaled(&self, decimals: u8) -> U256 {
        cast_decimals(self.value, Self::DECIMALS, decimals)
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    pub fn pow2(&self) -> Num {
        *self * *self
    }

    pub fn sqrt(&self) -> Num {
        ((self.value * Self::DENOMINATOR).integer_sqrt()).into()
    }

    // Round self value by using the given base.
    // self / base * base
    pub fn round(&self, base: Num) -> Num {
        (self.value / base.value * base.value).into()
    }
}

impl Mul<Self> for Num {
    type Output = Num;

    fn mul(self, rhs: Self) -> Self::Output {
        Num {
            value: mul_div256(self.value, rhs.value, Self::DENOMINATOR).expect("overflow"),
        }
    }
}

impl Div<Self> for Num {
    type Output = Num;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: mul_div256(self.value, Self::DENOMINATOR, rhs.value).expect("overflow"),
        }
    }
}

impl Rem<Self> for Num {
    type Output = Num;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value % rhs.value,
        }
    }
}

impl Add<Self> for Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.checked_add(rhs.value).expect("overflow"),
        }
    }
}

impl AddAssign<Self> for Num {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign<Self> for Num {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign<Self> for Num {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Sub<Self> for Num {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.checked_sub(rhs.value).expect("overflow"),
        }
    }
}

impl SubAssign<Self> for Num {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Debug for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", cast_integer_to_float(self.value, Self::DECIMALS))
    }
}

impl Sum for Num {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(0.into(), |acc, num| acc + num)
    }
}

impl Serialize for Num {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&cast_integer_to_float(self.value, Self::DECIMALS))
    }
}

impl<'de> Deserialize<'de> for Num {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(cast_float_to_integer(&s, Self::DECIMALS).into())
    }
}

impl From<Num> for U256 {
    fn from(value: Num) -> Self {
        U256::from(value.value)
    }
}

impl From<U256> for Num {
    fn from(value: U256) -> Self {
        Self {
            value: value.try_into().ok().expect("overflow"),
        }
    }
}

impl From<Num> for ethers::types::U256 {
    fn from(value: Num) -> Self {
        let mut b = [];
        value.value.to_big_endian(&mut b);
        ethers::types::U256::from_big_endian(&b)
    }
}

impl From<ethers::types::U256> for Num {
    fn from(value: ethers::types::U256) -> Self {
        let mut b = [];
        value.to_big_endian(&mut b);
        Self {
            value: U256::from_big_endian(&b),
        }
    }
}

impl From<u128> for Num {
    fn from(value: u128) -> Self {
        Num {
            value: value.into(),
        }
    }
}

impl From<Num> for u128 {
    fn from(value: Num) -> Self {
        value.value.as_u128()
    }
}

impl From<&str> for Num {
    fn from(value: &str) -> Self {
        cast_float_to_integer(value, Self::DECIMALS).into()
    }
}

impl From<String> for Num {
    fn from(value: String) -> Self {
        cast_float_to_integer(value.as_str(), Self::DECIMALS).into()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        ops::{Add, Div, Mul, Rem, Sub},
    };

    use super::*;

    #[test]
    pub fn test_sqrt() {
        assert_eq!(Num::from("16").sqrt(), Num::from("4"));
        assert_eq!(Num::from("0.16").sqrt(), Num::from("0.4"));
        assert_eq!(Num::from("0.0016").sqrt(), Num::from("0.04"));
    }

    #[test]
    pub fn test_from_str() {
        assert_eq!(Num::from("3.333").to_string(), "3.333");
        assert_eq!(Num::from("3").to_string(), "3");
        assert_eq!(Num::from("0.1").to_string(), "0.1");
        assert_eq!(Num::from("0.000001").to_string(), "0.000001");
    }

    #[test]
    pub fn test_normalize() {
        test((2_000130001, 9), (1_00000000, 9), "2");
        test((2_000130001, 9), (1, 4), "2.0001");
        test((2_000130001, 9), (1, 5), "2.00013");
        test((2_000130001, 9), (1, 22), "2.000130001");

        fn test(a: (u128, u8), b: (u128, u8), res: &str) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(&a.round(b).to_string(), res);
        }
    }

    #[test]
    pub fn test_iter_sum() {
        let init = vec![(2, 0), (2, 0), (2_00, 2)];
        let sum = init
            .iter()
            .map(|(v, d)| Num::with_decimals((*v).into(), *d as u8))
            .sum();
        assert_eq!(Num::with_decimals(6.into(), 0), sum);
    }

    #[test]
    pub fn test_scale() {
        assert_eq!(Num::with_decimals(10_100000.into(), 6).scaled(0), 10.into());
        assert_eq!(
            Num::with_decimals(10_100000.into(), 6).scaled(1),
            101.into()
        );
        assert_eq!(
            Num::with_decimals(10_100000.into(), 6).scaled(7),
            101000000.into()
        );
    }

    #[test]
    pub fn test_view() {
        assert_eq!(&Num::with_decimals(10_100000.into(), 6).to_string(), "10.1");
        assert_eq!(&Num::with_decimals(1.into(), 0).to_string(), "1");
        assert_eq!(
            &Num::with_decimals(3_141592653u128.into(), 9).to_string(),
            "3.141592653"
        );
    }

    #[test]
    pub fn test_convert() {
        assert_eq!(
            Num::with_decimals(10_100130.into(), 6).scaled(5),
            10_10013.into()
        );
        assert_eq!(
            Num::with_decimals(10_100133.into(), 6).scaled(6),
            10_100133.into()
        );
        assert_eq!(
            Num::with_decimals(10_100000.into(), 6).scaled(1),
            101.into()
        );
        assert_eq!(Num::with_decimals(10_100000.into(), 6).scaled(0), 10.into());

        assert_eq!(Num::with_decimals(1.into(), 0).scaled(5), 100000.into());
        assert_eq!(
            Num::with_decimals(3_141592653u128.into(), 9).scaled(6),
            3_141592.into()
        );
    }

    #[test]
    pub fn test_mul() {
        test((45, 0), (19, 0), "855");
        test((2_0, 1), (3, 0), "6");
        test((2_000000000, 9), (3_000000000, 9), "6");
        test((1_000000000, 9), (2, 0), "2");
        test((1_00000000, 8), (2_000, 3), "2");

        test((3_141592653, 9), (2_000, 3), "6.283185306");

        fn test(a: (u128, u8), b: (u128, u8), res: &str) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(&a.mul(b).to_string(), res);
        }
    }

    #[test]
    pub fn test_div() {
        test((6, 0), (2, 0), "3");
        test((4_000000000, 9), (2, 0), "2");
        test((3, 0), (2, 0), "1.5");
        test((15, 1), (2, 1), "7.5");
        test((62_000000, 6), (100_000000000, 9), "0.62");
        test((62_000000, 6), (100_000000000, 9), "0.62");
        test((6_283185306, 9), (2_000, 3), "3.141592653");

        fn test(a: (u128, u8), b: (u128, u8), res: &str) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(&a.div(b).to_string(), res);
        }
    }

    #[test]
    pub fn test_rem() {
        test((60, 0), (10, 0), "0");
        test((6, 0), (10, 0), "6");
        test((6, 1), (10, 0), "0.6");
        test((6, 2), (10, 0), "0.06");
        test((6, 2), (10, 4), "0");
        test((60000, 3), (10000, 2), "60");
        test((4_000000000, 9), (2, 0), "0");
        test((3, 0), (2, 0), "1");
        fn test(a: (u128, u8), b: (u128, u8), res: &str) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(&a.rem(b).to_string(), res);
        }
    }

    #[test]
    pub fn test_add() {
        test((1, 0), (2, 0), "3");
        test((1_000000000, 9), (2_000000000, 9), "3");
        test((1_000, 3), (2_000000000, 9), "3");

        fn test(a: (u128, u8), b: (u128, u8), res: &str) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(&a.add(b).to_string(), res);
        }
    }

    #[test]
    pub fn test_sub() {
        test((3, 0), (1, 0), "2");
        test((3_000000000, 9), (1_000000, 6), "2");
        test((3_000, 3), (2_00000, 5), "1");

        fn test(a: (u128, u8), b: (u128, u8), res: &str) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(&a.sub(b).to_string(), res);
        }
    }

    #[test]
    pub fn test() {
        let val_1 = Num::with_decimals(10000.into(), 24);
        let val_2 = Num::with_decimals(200000000.into(), 18);
        println!("{val_1}");
        println!("{val_2}");
        println!("{}", val_2 * val_1);
    }

    #[test]
    pub fn test_eq() {
        test((1, 0), (1, 0), true);
        test((10, 1), (1, 0), true);
        test((10, 1), (1_000000000, 9), true);
        test((10, 1), (1_000000001, 9), false);

        fn test(a: (u128, u8), b: (u128, u8), res: bool) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(a.eq(&b), res);
        }
    }

    #[test]
    pub fn test_cmp() {
        test((2, 0), (1, 0), Ordering::Greater);
        test((2, 0), (3, 0), Ordering::Less);
        test((3, 0), (3, 0), Ordering::Equal);

        test((2_00, 2), (1, 0), Ordering::Greater);
        test((2_00, 2), (3, 0), Ordering::Less);
        test((3_00, 2), (3, 0), Ordering::Equal);

        fn test(a: (u128, u8), b: (u128, u8), res: Ordering) {
            let a = Num::with_decimals(a.0.into(), a.1);
            let b = Num::with_decimals(b.0.into(), b.1);
            assert_eq!(a.cmp(&b), res);
        }
    }

    #[test]
    pub fn test_with_decimals() {
        assert_eq!(Num::with_decimals(1.into(), 22).value, 100.into());
    }

    #[test]
    pub fn test_is_zero() {
        test((1, 0), false);
        test((2, 0), false);
        test((1, 2), false);
        test((2, 2), false);

        test((0, 0), true);
        test((0, 1), true);
        test((0, 10), true);

        fn test(a: (u128, u8), res: bool) {
            let a = Num::with_decimals(a.0.into(), a.1);
            assert_eq!(a.is_zero(), res);
        }
    }
}
