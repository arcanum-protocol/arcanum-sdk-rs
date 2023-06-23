use super::settings::{BurnTxnParams, MintTxnParams, SwapTxnParams};
use crate::num::num::Num;

use super::adapter::MpAdapter;
use super::{settings::ActionSettings, Deadline, SidedQuantity, Slippage};

impl<A: MpAdapter> ActionSettings<A> {
    pub fn mint(mut self) -> Self {
        let adapter = self.adapter.as_mut().expect("adapter not set");
        let mut context = self.context.as_ref().expect("context not set").clone();
        let total_supply = self.total_supply.expect("total supply not set").clone();
        let mut asset = self.asset_in.as_ref().expect("asset in not set").clone();
        let shares;
        let amount_in_max;
        match self.quantity.expect("sending or receiving amount not set") {
            SidedQuantity::QuantityIn(amount_in) => {
                let amount_out = context
                    .mint(&mut asset, amount_in)
                    .expect("failed to calculate");
                shares = amount_out * asset.price * total_supply / context.total_current_usd_amount;
                amount_in_max = amount_in;
            }
            SidedQuantity::QuantityOut(share) => {
                shares = share;
                let amount_out =
                    share * context.total_current_usd_amount / asset.price / total_supply;
                let amount_in = context
                    .mint_rev(&mut asset, amount_out)
                    .expect("failed to calculate");
                amount_in_max = match self.slippage {
                    Some(s) => match s {
                        Slippage::Percent(p) => amount_in + amount_in * p,
                    },
                    None => amount_in,
                }
            }
        }
        let deadline = match self.deadline {
            Some(dl) => match dl {
                Deadline::Block(b) => b,
                Deadline::BlockInterval(i) => adapter.get_current_block() + i,
            },
            None => Num::ZERO,
        };
        self.mint_params = Some(MintTxnParams {
            pool_address: self
                .pool_address
                .as_ref()
                .expect("pool address not set")
                .to_owned(),
            asset_in_address: self
                .asset_in_address
                .as_ref()
                .expect("asset in address not set")
                .to_owned(),
            shares,
            amount_in_max,
            receiver_address: self
                .receiver_address
                .as_ref()
                .expect("receiver address not set")
                .to_owned(),
            deadline,
        });
        self
    }

    pub fn burn(mut self) -> Self {
        let adapter = self.adapter.as_mut().expect("adapter not set");
        let mut context = self.context.as_ref().expect("context not set").clone();
        let total_supply = self.total_supply.expect("total supply not set").clone();
        let mut asset = self.asset_out.as_ref().expect("asset out not set").clone();
        let shares;
        let amount_out_min;
        match self.quantity.expect("sending or receiving amount not set") {
            SidedQuantity::QuantityIn(share) => {
                shares = share;
                let amount_in =
                    share * context.total_current_usd_amount / asset.price / total_supply;
                let amount_out = context
                    .burn(&mut asset, amount_in)
                    .expect("failed to calculate");
                amount_out_min = match self.slippage {
                    Some(s) => match s {
                        Slippage::Percent(p) => amount_out + amount_out * p,
                    },
                    None => amount_out,
                }
            }
            SidedQuantity::QuantityOut(amount_out) => {
                let amount_in = context
                    .burn_rev(&mut asset, amount_out)
                    .expect("failed to calculate");
                shares = amount_in * asset.price * total_supply / context.total_current_usd_amount;
                amount_out_min = amount_out;
            }
        }
        let deadline = match self.deadline {
            Some(dl) => match dl {
                Deadline::Block(b) => b,
                Deadline::BlockInterval(i) => adapter.get_current_block() + i,
            },
            None => Num::ZERO,
        };
        self.burn_params = Some(BurnTxnParams {
            pool_address: self
                .pool_address
                .as_ref()
                .expect("pool address not set")
                .to_owned(),
            asset_out_address: self
                .asset_out_address
                .as_ref()
                .expect("asset out address not set")
                .to_owned(),
            shares,
            amount_out_min,
            receiver_address: self
                .receiver_address
                .as_ref()
                .expect("receiver address not set")
                .to_owned(),
            deadline,
        });
        self
    }

    pub fn swap(mut self) -> Self {
        let adapter = self.adapter.as_mut().expect("adapter not set");
        let mut context = self.context.as_ref().expect("context not set").clone();
        let total_supply = self.total_supply.expect("total supply not set").clone();
        let mut asset_out = self.asset_out.as_ref().expect("asset out not set").clone();
        let mut asset_in = self.asset_in.as_ref().expect("asset in not set").clone();
        let shares;
        let amount_out_min;
        let amount_in_max;
        match self.quantity.expect("sending or receiving amount not set") {
            SidedQuantity::QuantityIn(amount_in) => {
                let amount_out_mint = context
                    .mint(&mut asset_in, amount_in)
                    .expect("failed to calculate");

                shares = amount_out_mint * asset_in.price * total_supply
                    / context.total_current_usd_amount;

                let amount_in_burn =
                    shares * context.total_current_usd_amount / total_supply / asset_out.price;

                let amount_out = context
                    .burn(&mut asset_out, amount_in_burn)
                    .expect("failed to calculate");

                amount_out_min = match self.slippage {
                    Some(s) => match s {
                        Slippage::Percent(p) => amount_out + amount_out * p,
                    },
                    None => amount_out,
                };
                amount_in_max = amount_in;
            }
            SidedQuantity::QuantityOut(amount_out) => {
                let amount_in_burn = context
                    .burn_rev(&mut asset_out, amount_out)
                    .expect("failed to calculate");

                shares = amount_in_burn * asset_out.price * total_supply
                    / context.total_current_usd_amount;

                let amount_out_mint =
                    shares * context.total_current_usd_amount / total_supply / asset_in.price;

                let amount_in = context
                    .mint_rev(&mut asset_in, amount_out_mint)
                    .expect("failed to calculate");

                amount_in_max = match self.slippage {
                    Some(s) => match s {
                        Slippage::Percent(p) => amount_in + amount_in * p,
                    },
                    None => amount_out,
                };
                amount_out_min = amount_out;
            }
        }
        let deadline = match self.deadline {
            Some(dl) => match dl {
                Deadline::Block(b) => b,
                Deadline::BlockInterval(i) => adapter.get_current_block() + i,
            },
            None => Num::ZERO,
        };
        self.swap_params = Some(SwapTxnParams {
            pool_address: self
                .pool_address
                .as_ref()
                .expect("pool address not set")
                .to_owned(),
            asset_out_address: self
                .asset_out_address
                .as_ref()
                .expect("asset out address not set")
                .to_owned(),
            asset_in_address: self
                .asset_in_address
                .as_ref()
                .expect("asset in address not set")
                .to_owned(),
            shares,
            amount_out_min,
            amount_in_max,
            receiver_address: self
                .receiver_address
                .as_ref()
                .expect("receiver address not set")
                .to_owned(),
            deadline,
        });
        self
    }
}
