use crate::multipool_math::actions::adapter::MpAdapter;
use ethers::prelude::*;
use std::sync::Arc;

pub struct EthersConnection<M: Middleware, S: Signer> {
    provider: M,
    signer: Arc<SignerMiddleware<M, S>>,
}

abigen!(
    Multipool,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function getAsset(address asset) external view returns (Asset)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string memory)
        function transfer(address to, uint256 amount) external returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
    ]"#,
);

abigen!(
    ERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string memory)
        function transfer(address to, uint256 amount) external returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
    ]"#,
);

impl<M: Middleware, S: Signer> EthersConnection<M, S> {
    fn new(middleware: M, signer: S) -> Self {
        Self {
            provider: middleware,
            signer: Arc::new(SignerMiddleware::new(middleware, signer)),
        }
    }
}

impl<M: Middleware, S: Signer> MpAdapter for EthersConnection<M, S> {
    type Error = ();
    type MintTxnResult = ();
    type BurnTxnResult = ();
    type SwapTxnResult = ();

    fn get_asset(
        &mut self,
        address: &str,
        asset: &str,
    ) -> Result<crate::multipool_math::MpAsset, Self::Error> {
        let pool_address = hex::decode(address).expect("can't parse pool address");
        Multipool::new(address.parse().unwrap(), self.signer).
    }
    fn get_context(
        &mut self,
        address: &str,
    ) -> Result<crate::multipool_math::MpContext, Self::Error> {
    }
    fn get_total_supply(&mut self, address: &str) -> Result<crate::num::num::Num, Self::Error> {}
    fn get_current_block(&mut self) -> crate::num::num::Num {}
    fn transact_mint(
        &mut self,
        params: &crate::multipool_math::actions::settings::MintTxnParams,
    ) -> Self::MintTxnResult {
    }
    fn transact_burn(
        &mut self,
        params: &crate::multipool_math::actions::settings::BurnTxnParams,
    ) -> Self::BurnTxnResult {
    }
    fn transact_swap(
        &mut self,
        params: &crate::multipool_math::actions::settings::SwapTxnParams,
    ) -> Self::SwapTxnResult {
    }
}
