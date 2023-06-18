use super::uint256::U256;

pub(crate) mod strings;

#[inline]
pub fn pow10(exp: u8) -> U256 {
    U256::from(10).pow(U256::from(exp))
}

pub fn cast_decimals(value: U256, prev: u8, new: u8) -> U256 {
    use std::cmp::Ordering::*;
    match prev.cmp(&new) {
        Equal => value,
        Greater => value.checked_div(pow10(prev - new)).expect("overflow"),
        Less => value.checked_mul(pow10(new - prev)).expect("overflow"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casting_decimals() {
        assert_eq!(cast_decimals(1.into(), 22, 24), U256::from(100));
    }
}
