use crate::num::num::Num;

use super::adapter::MpAdapter;
use super::{Deadline, SidedQuantity, Slippage};
use crate::multipool_math::{MpAsset, MpContext};

#[derive(Default, Debug)]
pub struct ActionSettings<A: MpAdapter> {
    pub quantity: Option<SidedQuantity>,
    pub slippage: Option<Slippage>,
    pub asset_in: Option<MpAsset>,
    pub asset_out: Option<MpAsset>,
    pub context: Option<MpContext>,
    pub total_supply: Option<Num>,
    pub pool_address: Option<String>,
    pub asset_in_address: Option<String>,
    pub asset_out_address: Option<String>,
    pub receiver_address: Option<String>,
    pub deadline: Option<Deadline>,
    // calculations outcome to send to router
    pub mint_params: Option<MintTxnParams>,
    pub burn_params: Option<BurnTxnParams>,
    pub swap_params: Option<SwapTxnParams>,
    // adapter that lets you operate data fetching and other things
    pub adapter: Option<A>,
}

#[derive(Clone, Debug)]
pub struct MintTxnParams {
    pub pool_address: String,
    pub asset_in_address: String,
    pub shares: Num,
    pub amount_in_max: Num,
    pub receiver_address: String,
    pub deadline: Num,
}

#[derive(Clone, Debug)]
pub struct BurnTxnParams {
    pub pool_address: String,
    pub asset_out_address: String,
    pub shares: Num,
    pub amount_out_min: Num,
    pub receiver_address: String,
    pub deadline: Num,
}

#[derive(Clone, Debug)]
pub struct SwapTxnParams {
    pub pool_address: String,
    pub asset_in_address: String,
    pub asset_out_address: String,
    pub shares: Num,
    pub amount_in_max: Num,
    pub amount_out_min: Num,
    pub receiver_address: String,
    pub deadline: Num,
}
