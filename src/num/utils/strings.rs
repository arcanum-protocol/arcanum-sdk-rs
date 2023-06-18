use crate::num::uint256::U256;

use super::pow10;

pub fn cast_integer_to_float(num: U256, decimals: u8) -> String {
    let decimals = decimals as usize;
    let mut arr: Vec<char> = num.to_string().chars().collect();
    let arr = if arr.len() > decimals {
        arr.insert(arr.len() - decimals, '.');
        arr.reverse();
        arr
    } else {
        let mut prefix = vec!['0', '.'];
        prefix.append(&mut vec!['0'; decimals - arr.len()]);
        prefix.append(&mut arr);
        prefix.reverse();
        prefix
    };
    let mut v = arr
        .into_iter()
        .skip_while(|v| v.eq(&'0'))
        .collect::<Vec<char>>();
    if v[0] == '.' {
        v.remove(0);
    }
    v.reverse();
    v.into_iter().collect()
}

pub fn cast_float_to_integer(value: &str, decimals: u8) -> U256 {
    let sp = value.split('.').collect::<Vec<&str>>();
    let prev_decimals = sp.get(1).map(|v| v.len()).unwrap_or(0);
    let val = U256::from_dec_str(sp[0]).unwrap() * pow10(prev_decimals as u8)
        + U256::from_dec_str(sp.get(1).unwrap_or(&"0")).unwrap();
    super::cast_decimals(val, prev_decimals as u8, decimals)
}

#[cfg(test)]
pub mod tests {
    use super::super::pow10;

    use super::*;

    #[test]
    pub fn test_casts_1() {
        let res = cast_integer_to_float(U256::from(10), 6);
        assert_eq!("0.00001", res);
    }

    #[test]
    pub fn test_casts_2() {
        let res = cast_integer_to_float(U256::from(100000), 6);
        assert_eq!("0.1", res);
    }

    #[test]
    pub fn test_casts_3() {
        let res = cast_integer_to_float(U256::from(1000000000), 6);
        assert_eq!("1000", res);
    }

    #[test]
    pub fn test_casts_4() {
        let res = cast_integer_to_float(U256::from(1000000), 6);
        assert_eq!("1", res);
    }

    #[test]
    pub fn test_casts_5() {
        let res = cast_integer_to_float(U256::from(1100000), 6);
        assert_eq!("1.1", res);
    }

    #[test]
    pub fn test_casts_6() {
        let res = cast_integer_to_float(U256::from(1100001), 6);
        assert_eq!("1.100001", res);
    }

    #[test]
    pub fn test_casts_7() {
        let res = cast_float_to_integer("10", 24);
        assert_eq!(U256::from(10) * pow10(24), res);
    }

    #[test]
    pub fn test_casts_8() {
        let res = cast_float_to_integer("10.1", 24);
        assert_eq!(U256::from(101) * pow10(23), res);
    }

    #[test]
    pub fn test_casts_9() {
        let res = cast_float_to_integer("10.1", 6);
        assert_eq!(U256::from(101) * pow10(5), res);
    }

    #[test]
    pub fn test_casts_10() {
        assert_eq!(
            U256::from(10111111),
            cast_float_to_integer("10.11111111", 6)
        );
    }
}
