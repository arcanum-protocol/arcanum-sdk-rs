use crate::num::num::Num;
use std::collections::HashMap;

//use super::{burn::Burn, mint::Mint, swap::Swap};
use super::settings::{ActionSettings, BurnTxnParams, MintTxnParams, SwapTxnParams};
use crate::multipool_math::{MpAsset, MpContext};

pub trait MpAdapter: Sized {
    type Error;
    type MintTxnResult;
    type BurnTxnResult;
    type SwapTxnResult;

    fn get_context(&mut self, address: &str) -> Result<MpContext, Self::Error>;
    fn get_total_supply(&mut self, address: &str) -> Result<Num, Self::Error>;
    fn get_asset(&mut self, address: &str, asset: &str) -> Result<MpAsset, Self::Error>;
    fn get_current_block(&mut self) -> Num;

    fn transact_mint(&mut self, params: &MintTxnParams) -> Self::MintTxnResult;
    fn transact_burn(&mut self, params: &BurnTxnParams) -> Self::BurnTxnResult;
    fn transact_swap(&mut self, params: &SwapTxnParams) -> Self::SwapTxnResult;

    fn configure(self) -> ActionSettings<Self> {
        ActionSettings::new(self)
    }
}

pub struct MockedAdapter {
    pub assets: HashMap<String, MpAsset>,
    pub context: MpContext,
    pub total_supply: Num,
    pub current_block: Num,
}

impl MpAdapter for MockedAdapter {
    type Error = ();
    type MintTxnResult = MintTxnParams;
    type BurnTxnResult = BurnTxnParams;
    type SwapTxnResult = SwapTxnParams;

    fn get_current_block(&mut self) -> Num {
        self.current_block
    }

    fn get_context(&mut self, _address: &str) -> Result<MpContext, Self::Error> {
        Ok(self.context.to_owned())
    }
    fn get_total_supply(&mut self, _address: &str) -> Result<Num, Self::Error> {
        Ok(self.total_supply)
    }
    fn get_asset(&mut self, _address: &str, asset: &str) -> Result<MpAsset, Self::Error> {
        Ok(self
            .assets
            .get(asset)
            .expect("no such asset in mock")
            .to_owned())
    }
    fn transact_mint(&mut self, params: &MintTxnParams) -> Self::MintTxnResult {
        params.to_owned()
    }

    fn transact_burn(&mut self, params: &BurnTxnParams) -> Self::BurnTxnResult {
        params.to_owned()
    }

    fn transact_swap(&mut self, params: &SwapTxnParams) -> Self::SwapTxnResult {
        params.to_owned()
    }
}
