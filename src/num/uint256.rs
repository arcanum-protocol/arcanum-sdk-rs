#[allow(clippy::ptr_offset_with_cast)]
#[allow(clippy::assign_op_pattern)]
pub mod uint_256 {
    use borsh::{self, BorshDeserialize, BorshSerialize};
    use uint::construct_uint;
    construct_uint! {
        #[derive(BorshSerialize, BorshDeserialize)] pub struct U256(4);
    }
}
pub use uint_256::U256;

//TODO: remove this muldiv and change everywhere to be muldiv256 with chaning it's name to this
pub fn mul_div<A: Into<U256>, B: Into<U256>, C: Into<U256>>(a: A, b: B, c: C) -> Option<u128> {
    let a = a.into();
    let b = b.into();
    let c = c.into();

    if c == U256::from(0) {
        Some(0)
    } else {
        a.checked_mul(b)
            .and_then(|v| v.checked_div(c))
            .map(TryInto::try_into)
            .transpose()
            .ok()
            .flatten()
    }
}

pub fn mul_div256<A: Into<U256>, B: Into<U256>, C: Into<U256>>(a: A, b: B, c: C) -> Option<U256> {
    let a = a.into();
    let b = b.into();
    let c = c.into();

    if c == U256::from(0) {
        Some(U256::from(0))
    } else {
        a.checked_mul(b).and_then(|val| val.checked_div(c))
    }
}
