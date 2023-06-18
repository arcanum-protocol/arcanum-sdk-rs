#[allow(clippy::ptr_offset_with_cast)]
#[allow(clippy::assign_op_pattern)]
pub mod uint_512 {
    use borsh::{self, BorshDeserialize, BorshSerialize};
    use uint::construct_uint;
    construct_uint! {
        #[derive(BorshSerialize, BorshDeserialize)] pub struct U512(8);
    }
}
pub use uint_512::U512;

pub fn mul_div512<A: Into<U512>, B: Into<U512>, C: Into<U512>>(a: A, b: B, c: C) -> Option<U512> {
    let a = a.into();
    let b = b.into();
    let c = c.into();

    if c == U512::from(0) {
        Some(U512::from(0))
    } else {
        a.checked_mul(b).and_then(|val| val.checked_div(c))
    }
}
