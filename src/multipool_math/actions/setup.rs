use crate::num::num::Num;
use super::adapter::MpAdapter;
use super::{settings::ActionSettings, Deadline, SidedQuantity, Slippage};

impl<A: MpAdapter> ActionSettings<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            quantity: None,
            slippage: None,
            asset_in: None,
            asset_out: None,
            context: None,
            total_supply: None,
            pool_address: None,
            asset_in_address: None,
            asset_out_address: None,
            receiver_address: None,
            deadline: None,
            mint_params: None,
            burn_params: None,
            swap_params: None,
            adapter: Some(adapter),
            router_address: None
        }
    }

    pub fn router_address(mut self, val: String) -> Self {
        self.router_address = Some(val);
        self
    }

    pub fn amount_in<V: Into<Num>>(mut self, val: V) -> Self {
        self.quantity = Some(SidedQuantity::QuantityIn(val.into()));
        self
    }

    pub fn amount_out<V: Into<Num>>(mut self, val: V) -> Self {
        self.quantity = Some(SidedQuantity::QuantityOut(val.into()));
        self
    }

    pub fn slippage_percent<V: Into<Num>>(mut self, val: V) -> Self {
        self.slippage = Some(Slippage::Percent(val.into()));
        self
    }

    pub fn pool<V: Into<String>>(mut self, val: V) -> Self {
        self.pool_address = Some(val.into());
        self
    }

    pub fn asset_in<V: Into<String>>(mut self, val: V) -> Self {
        self.asset_in_address = Some(val.into());
        self
    }

    pub fn asset_out<V: Into<String>>(mut self, val: V) -> Self {
        self.asset_out_address = Some(val.into());
        self
    }

    pub fn receiver<V: Into<String>>(mut self, val: V) -> Self {
        self.receiver_address = Some(val.into());
        self
    }

    pub fn until_block<V: Into<Num>>(mut self, val: V) -> Self {
        self.deadline = Some(Deadline::Block(val.into()));
        self
    }

    pub fn blocks_to_live<V: Into<Num>>(mut self, val: V) -> Self {
        self.deadline = Some(Deadline::BlockInterval(val.into()));
        self
    }

    pub async fn fetch(mut self) -> Self {
        let adapter = self.adapter.as_mut().expect("adapter not set");
        self.context = adapter
            .get_trading_context(self.pool_address.as_ref().expect("pool address not set"))
            .await
            .ok();
        self.total_supply = adapter
            .get_total_supply(&self.pool_address.as_ref().expect("pool address not set"))
            .await
            .ok();
        self.asset_in = adapter
            .get_asset(
                &self.pool_address.as_ref().expect("pool address not set"),
                &self
                    .asset_in_address
                    .as_ref()
                    .expect("asset in address not set"),
            )
            .await
            .ok();
        self.asset_out = adapter
            .get_asset(
                &self.pool_address.as_ref().expect("pool address not set"),
                &self
                    .asset_in_address
                    .as_ref()
                    .expect("asset out address not set"),
            )
            .await
            .ok();
        self
    }

    pub async fn send_mint(&mut self) -> A::MintTxnResult {
        let adapter = self.adapter.as_mut().expect("adapter not set");
        adapter
            .transact_mint(
                self.router_address.as_ref().expect("Router address is not set"),
                self.mint_params
                    .as_ref()
                    .expect("params are not calculated"),
            )
            .await
    }

    pub async fn send_burn(&mut self) -> A::BurnTxnResult {
        let adapter = self.adapter.as_mut().expect("adapter not set");
        adapter
            .transact_burn(
                self.router_address.as_ref().expect("Router address is not set"),
                self.burn_params
                    .as_ref()
                    .expect("params are not calculated"),
            )
            .await
    }

    pub async fn send_swap(&mut self) -> A::SwapTxnResult {
        let adapter = self.adapter.as_mut().expect("adapter not set");
        adapter
            .transact_swap(
                self.router_address.as_ref().expect("Router address is not set"),
                self.swap_params
                    .as_ref()
                    .expect("params are not calculated"),
            )
            .await
    }
}
