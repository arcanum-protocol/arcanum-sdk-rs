use super::{num::Num, uint256::U256};
use borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{
    de,
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    cmp::{max, min, Ordering},
    fmt::{Debug, Display, Formatter},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Signed decimal implementation.
#[derive(Eq, BorshSerialize, BorshDeserialize, Copy, Clone, Default)]
pub struct SNum {
    /// Absolute value.
    value: Num,
    /// Sign.
    is_negative: bool,
}

impl SNum {
    pub const MAX: Self = Self {
        value: Num::MAX,
        is_negative: false,
    };

    pub const MIN: Self = Self {
        value: Num::MAX,
        is_negative: true,
    };

    pub const ONE: Self = Self {
        value: Num::ONE,
        is_negative: false,
    };

    pub const ZERO: Self = Self {
        value: Num::ZERO,
        is_negative: false,
    };

    /// Returns true if the value is negative, false otherwise.
    pub fn is_negative(&self) -> bool {
        self.is_negative
    }

    /// Returns absolute value.
    pub fn abs(&self) -> Num {
        self.value
    }

    /// Round number.
    pub fn round(&self, base: Num) -> SNum {
        SNum {
            value: self.value.round(base),
            is_negative: self.is_negative,
        }
    }

    pub fn pow(&self, exp: U256) -> SNum {
        self.value.pow(exp).into()
    }

    pub fn sqrt(&self) -> SNum {
        self.value.sqrt().into()
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.into()
    }
}

impl From<Num> for SNum {
    fn from(value: Num) -> Self {
        Self {
            value,
            is_negative: false,
        }
    }
}

impl Add<Self> for SNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.is_negative, rhs.is_negative) {
            (true, true) => Self {
                value: self.value + rhs.value,
                is_negative: true,
            },
            (true, false) => Self {
                value: max(self.value, rhs.value) - min(self.value, rhs.value),
                is_negative: self.value > rhs.value,
            },
            (false, true) => Self {
                value: max(self.value, rhs.value) - min(self.value, rhs.value),
                is_negative: self.value < rhs.value,
            },
            (false, false) => Self {
                value: self.value + rhs.value,
                is_negative: false,
            },
        }
    }
}

impl Sub<Self> for SNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.is_negative, rhs.is_negative) {
            (true, true) => Self {
                value: max(self.value, rhs.value) - min(self.value, rhs.value),
                is_negative: self.value > rhs.value,
            },
            (true, false) => Self {
                value: self.value + rhs.value,
                is_negative: true,
            },
            (false, true) => Self {
                value: self.value + rhs.value,
                is_negative: false,
            },
            (false, false) => Self {
                value: max(self.value, rhs.value) - min(self.value, rhs.value),
                is_negative: self.value < rhs.value,
            },
        }
    }
}

impl Mul<Self> for SNum {
    type Output = SNum;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            is_negative: self.is_negative ^ rhs.is_negative,
        }
    }
}

impl Div<Self> for SNum {
    type Output = SNum;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value,
            is_negative: self.is_negative ^ rhs.is_negative,
        }
    }
}

impl PartialEq for SNum {
    fn eq(&self, other: &Self) -> bool {
        self.is_zero() && other.is_zero()
            || self.value == other.value && self.is_negative == other.is_negative
    }
}

impl PartialOrd for SNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }
        match (self.is_negative, other.is_negative) {
            (false, false) => self.value.partial_cmp(&other.value),
            (true, true) => other.value.partial_cmp(&self.value),
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
        }
    }
}
impl Ord for SNum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("Unreachable signed comparasion")
    }
}

impl Sum for SNum {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |sum, d| sum + d)
    }
}

impl Debug for SNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:?}",
            if self.is_negative && !self.value.is_zero() {
                "-"
            } else {
                ""
            },
            self.value
        )
    }
}

impl From<&str> for SNum {
    fn from(value: &str) -> Self {
        let (is_negative, value) = if let Some(num) = value.strip_prefix('-') {
            (true, num)
        } else {
            (false, value)
        };

        let value: Num = value.into();

        SNum { value, is_negative }
    }
}

impl Display for SNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.is_negative && !self.value.is_zero() {
                "-"
            } else {
                ""
            },
            self.value
        )
    }
}

impl SubAssign<Self> for SNum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl AddAssign<Self> for SNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign<Self> for SNum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign<Self> for SNum {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for SNum {
    type Output = SNum;

    fn neg(mut self) -> Self::Output {
        self.is_negative = !self.is_negative;
        self
    }
}

impl Serialize for SNum {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct SignedDecimalVisitor;

impl<'de> Visitor<'de> for SignedDecimalVisitor {
    type Value = SNum;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "an signed decimal")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let (is_negative, value) = if let Some(num) = v.strip_prefix('-') {
            (true, num)
        } else {
            (false, v)
        };

        let value: Num = value.into();

        Ok(SNum { value, is_negative })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de> Deserialize<'de> for SNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(SignedDecimalVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ord() {
        use std::cmp::Ordering;

        let result = SNum::from("1").partial_cmp(&SNum::from("2"));
        assert_eq!(result, Some(Ordering::Less));
        let result = SNum::from("1").partial_cmp(&SNum::from("1"));
        assert_eq!(result, Some(Ordering::Equal));
        let result = SNum::from("2").partial_cmp(&SNum::from("1"));
        assert_eq!(result, Some(Ordering::Greater));

        let result = SNum::from("-1").partial_cmp(&SNum::from("2"));
        assert_eq!(result, Some(Ordering::Less));
        let result = SNum::from("-1").partial_cmp(&SNum::from("1"));
        assert_eq!(result, Some(Ordering::Less));
        let result = SNum::from("-2").partial_cmp(&SNum::from("1"));
        assert_eq!(result, Some(Ordering::Less));

        let result = SNum::from("1").partial_cmp(&SNum::from("-2"));
        assert_eq!(result, Some(Ordering::Greater));
        let result = SNum::from("1").partial_cmp(&SNum::from("-1"));
        assert_eq!(result, Some(Ordering::Greater));
        let result = SNum::from("2").partial_cmp(&SNum::from("-1"));
        assert_eq!(result, Some(Ordering::Greater));

        assert_eq!(SNum::ZERO, -SNum::ZERO);
    }

    #[test]
    pub fn test_add() {
        test_case((10, true), (15, true), (25, true));
        test_case((10, true), (15, false), (5, false));
        test_case((15, true), (10, false), (5, true));
        test_case((10, false), (15, true), (5, true));
        test_case((15, false), (10, true), (5, false));
        test_case((10, false), (15, false), (25, false));

        fn test_case(a: (u128, bool), b: (u128, bool), expected: (u128, bool)) {
            let mut v1: SNum = Num::with_decimals(a.0.into(), 0).into();
            if a.1 {
                v1 = -v1;
            }
            let mut v2: SNum = Num::with_decimals(b.0.into(), 0).into();
            if b.1 {
                v2 = -v2;
            }
            let mut v3: SNum = Num::with_decimals(expected.0.into(), 0).into();
            if expected.1 {
                v3 = -v3;
            }
            assert_eq!(v1 + v2, v3);
            assert_eq!(v2 + v1, v3);
        }
    }

    #[test]
    pub fn test_sub() {
        test_case((10, true), (15, true), (5, false));
        test_case((15, true), (10, true), (5, true));
        test_case((10, true), (15, false), (25, true));
        test_case((10, false), (15, true), (25, false));
        test_case((10, false), (15, false), (5, true));
        test_case((15, false), (10, false), (5, false));

        fn test_case(a: (u128, bool), b: (u128, bool), expected: (u128, bool)) {
            let mut v1: SNum = Num::with_decimals(a.0.into(), 0).into();
            if a.1 {
                v1 = -v1;
            }
            let mut v2: SNum = Num::with_decimals(b.0.into(), 0).into();
            if b.1 {
                v2 = -v2;
            }
            let mut v3: SNum = Num::with_decimals(expected.0.into(), 0).into();
            if expected.1 {
                v3 = -v3;
            }
            assert_eq!(v1 - v2, v3);
        }
    }

    #[test]
    pub fn test_serde() {
        fn test_serde(num: SNum) {
            let encoded = serde_json::to_string(&num).unwrap();
            assert_eq!(serde_json::from_str::<SNum>(&encoded).unwrap(), num);
        }

        test_serde(SNum::ZERO);
        test_serde(SNum::ONE);
        test_serde(-SNum::ONE);
        test_serde(SNum::MAX);
        test_serde(SNum::MIN);

        test_serde("3.14159".into());
        test_serde("-3.14159".into());
    }
}
