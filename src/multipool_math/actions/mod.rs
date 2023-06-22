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

#[derive(Clone, Copy, Debug)]
pub enum Slippage {
    Percent(Num),
    Price(Num),
}

#[derive(Clone, Copy, Debug)]
pub enum Deadline {
    BlockInterval(Num),
    Block(Num),
}
