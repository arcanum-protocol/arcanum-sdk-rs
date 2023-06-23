pub mod adapter;
pub mod computation;
pub mod settings;
pub mod setup;
#[cfg(test)]
pub mod tests;

use crate::num::num::Num;

#[derive(Clone, Copy, Debug)]
pub enum SidedQuantity {
    QuantityIn(Num),
    QuantityOut(Num),
}

/// Value specifies slippage
/// note: currently only percent specification is supported but there
/// might be made amountIn/amountOut (price) parameter constrained
/// by maximum and minimum
#[derive(Clone, Copy, Debug)]
pub enum Slippage {
    Percent(Num),
}

#[derive(Clone, Copy, Debug)]
pub enum Deadline {
    BlockInterval(Num),
    Block(Num),
}
