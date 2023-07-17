use crate::num::num::Num;
use std::collections::HashMap;

use super::settings::{ActionSettings, MintBurnTxnParams, SwapTxnParams};
use crate::multipool_math::{MpAsset, MpContext};

#[async_trait::async_trait]
pub trait MpAdapter: Sized {
    type Error;
    type MintTxnResult;
    type BurnTxnResult;
    type SwapTxnResult;

    async fn get_trading_context(&mut self, address: &str) -> Result<MpContext, Self::Error>;
    async fn get_mint_context(&mut self, address: &str) -> Result<MpContext, Self::Error>;
    async fn get_burn_context(&mut self, address: &str) -> Result<MpContext, Self::Error>;
    async fn get_total_supply(&mut self, address: &str) -> Result<Num, Self::Error>;
    async fn get_asset(&mut self, address: &str, asset: &str) -> Result<MpAsset, Self::Error>;
    async fn get_current_block(&mut self) -> Num;

    async fn transact_mint(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::MintTxnResult;
    async fn transact_mint_reversed(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::MintTxnResult;
    async fn transact_burn(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::BurnTxnResult;
    async fn transact_burn_reversed(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::BurnTxnResult;
    async fn transact_swap(&mut self, router_address: &str, params: &SwapTxnParams) -> Self::SwapTxnResult;
    async fn transact_swap_reversed(&mut self, router_address: &str, params: &SwapTxnParams) -> Self::SwapTxnResult;

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

#[async_trait::async_trait]
impl MpAdapter for MockedAdapter {
    type Error = ();
    type MintTxnResult = MintBurnTxnParams;
    type BurnTxnResult = MintBurnTxnParams;
    type SwapTxnResult = SwapTxnParams;

    async fn get_current_block(&mut self) -> Num {
        self.current_block
    }

    async fn get_trading_context(&mut self, _address: &str) -> Result<MpContext, Self::Error> {
        Ok(self.context.to_owned())
    }

    async fn get_burn_context(&mut self, _address: &str) -> Result<MpContext, Self::Error> {
        Ok(self.context.to_owned())
    }

    async fn get_mint_context(&mut self, _address: &str) -> Result<MpContext, Self::Error> {
        Ok(self.context.to_owned())
    }

    async fn get_total_supply(&mut self, _address: &str) -> Result<Num, Self::Error> {
        Ok(self.total_supply)
    }

    async fn get_asset(&mut self, _address: &str, asset: &str) -> Result<MpAsset, Self::Error> {
        Ok(self
            .assets
            .get(asset)
            .expect("no such asset in mock")
            .to_owned())
    }

    async fn transact_mint(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::MintTxnResult {
        params.to_owned()
    }

    async fn transact_burn(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::BurnTxnResult {
        params.to_owned()
    }

    async fn transact_swap(&mut self, router_address: &str, params: &SwapTxnParams) -> Self::SwapTxnResult {
        params.to_owned()
    }

    async fn transact_mint_reversed(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::MintTxnResult {
        params.to_owned()
    }

    async fn transact_burn_reversed(&mut self, router_address: &str, params: &MintBurnTxnParams) -> Self::BurnTxnResult {
        params.to_owned()
    }

    async fn transact_swap_reversed(&mut self, router_address: &str, params: &SwapTxnParams) -> Self::SwapTxnResult {
        params.to_owned()
    }
}
