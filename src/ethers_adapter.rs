use crate::multipool_math::actions::adapter::MpAdapter;
use ethers::prelude::*;
use std::sync::Arc;

pub struct EthersConnection<M: Middleware, S: Signer> {
    provider: M,
    signer: Arc<SignerMiddleware<M, S>>,
}

abigen!(
    ERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string memory)
        function totalSupply() external view returns (uint256)
        function transfer(address to, uint256 amount) external returns (bool)
        function approve(address to, uint256 amount) external returns (bool)
        function allowance(address from, address to) external view returns (uint256)
        event Transfer(address indexed from, address indexed to, uint256 value)
    ]"#,
    derives(serde::Deserialize, serde::Serialize);
    Multipool,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function getTradingContext() external view returns (uint256, uint256, uint256, uint256, uint256, uint256)
        function getMintContext() external view returns (uint256, uint256, uint256, uint256, uint256, uint256)
        function getBurnContext() external view returns (uint256, uint256, uint256, uint256, uint256, uint256)
        function getAssets(address asset) external view returns (uint256, uint256, uint256, uint256, uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string memory)
        function transfer(address to, uint256 amount) external returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
    ]"#,
    derives(serde::Deserialize, serde::Serialize);
    MultipoolRouter,
    r#"[
        function mintWithSharesOut(address _pool, address _asset, uint256 _sharesOut, uint _amountInMax, address _to, uint deadline) public
        function mintWithAmountIn(address _pool, address _asset, uint _amountIn, uint _sharesOutMin, address _to, uint deadline) public
        function burnWithSharesIn(address _pool, address _asset, uint _sharesIn, uint _amountOutMin, address _to, uint deadline) public
        function burnWithAmountOut(address _pool, address _asset, uint _amountOut, uint _sharesInMax, address _to, uint deadline) public
        function swap(address _pool, address _assetIn, address _assetOut, uint _amountInMax, uint _amountOutMin, uint _shares, address _to, uint deadline) public 
        function swapWithAmountIn(address _pool, address _assetIn, address _assetOut, uint _amountIn, uint _amountOutMin, address _to, uint deadline) public
        function swapWithAmountOut(address _pool, address _assetIn, address _assetOut, uint _amountOut, uint _amountInMax, address _to, uint deadline) public
    ]"#,
    derives(serde::Deserialize, serde::Serialize);
);

impl<M: Middleware + Clone, S: Signer> EthersConnection<M, S> {
    pub fn new(middleware: M, signer: S) -> Self {
        Self {
            provider: middleware.clone(),
            signer: Arc::new(SignerMiddleware::new(middleware, signer)),
        }
    }
}

pub enum QueryError {
    IvalidAddress,
    FailedQuery,
}

#[async_trait::async_trait]
impl<M: Middleware, S: Signer> MpAdapter for EthersConnection<M, S> {
    type Error = QueryError;
    type MintTxnResult = Result<(), anyhow::Error>;
    type BurnTxnResult = Result<(), anyhow::Error>;
    type SwapTxnResult = Result<(), anyhow::Error>;

    async fn get_asset(
        &mut self,
        address: &str,
        asset: &str,
    ) -> Result<crate::multipool_math::MpAsset, Self::Error> {
        let address: Address = address.parse().map_err(|_e| QueryError::IvalidAddress)?;
        let asset: Address = asset.parse().map_err(|_e| QueryError::IvalidAddress)?;
        Multipool::new(address, self.signer.clone())
            .get_assets(asset)
            .call()
            .await
            .map(|a| a.into())
            .map_err(|_| QueryError::FailedQuery)
    }

    async fn get_trading_context(
        &mut self,
        address: &str,
    ) -> Result<crate::multipool_math::MpContext, Self::Error> {
        let address: Address = address.parse().map_err(|_e| QueryError::IvalidAddress)?;
        Multipool::new(address, self.signer.clone())
            .get_trading_context()
            .call()
            .await
            .map(|c| c.into())
            .map_err(|_| QueryError::FailedQuery)
    }

    async fn get_burn_context(
        &mut self,
        address: &str,
    ) -> Result<crate::multipool_math::MpContext, Self::Error> {
        let address: Address = address.parse().map_err(|_e| QueryError::IvalidAddress)?;
        Multipool::new(address, self.signer.clone())
            .get_burn_context()
            .call()
            .await
            .map(|c| c.into())
            .map_err(|_| QueryError::FailedQuery)
    }

    async fn get_mint_context(
        &mut self,
        address: &str,
    ) -> Result<crate::multipool_math::MpContext, Self::Error> {
        let address: Address = address.parse().map_err(|_e| QueryError::IvalidAddress)?;
        Multipool::new(address, self.signer.clone())
            .get_mint_context()
            .call()
            .await
            .map(|c| c.into())
            .map_err(|_| QueryError::FailedQuery)
    }

    async fn get_total_supply(
        &mut self,
        address: &str,
    ) -> Result<crate::num::num::Num, Self::Error> {
        let address: Address = address.parse().map_err(|_e| QueryError::IvalidAddress)?;
        ERC20::new(address, self.signer.clone())
            .total_supply()
            .call()
            .await
            .map(|b| b.into())
            .map_err(|_| QueryError::FailedQuery)
    }

    async fn get_current_block(&mut self) -> crate::num::num::Num {
        unimplemented!()
    }

    async fn transact_mint(
        &mut self,
        router_address: &str,
        params: &crate::multipool_math::actions::settings::MintBurnTxnParams,
    ) -> Self::MintTxnResult {
        let router_address: Address = router_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        let pool_address: Address = params.pool_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        ERC20::new(pool_address, self.signer.clone())
            .approve(router_address, params.amount.into())
            .call()
            .await      
            .map_err(|_| anyhow::anyhow!("Failed to mint"))?;
        MultipoolRouter::new(router_address, self.signer.clone())
            .mint_with_shares_out(
                pool_address,
                params.asset_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.shares.into(),
                params.amount.into(),
                params.receiver_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.deadline.into()
            )
            .call()
            .await
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("Failed to mint"))
    }

    async fn transact_mint_reversed(
        &mut self,
        router_address: &str,
        params: &crate::multipool_math::actions::settings::MintBurnTxnParams,
    ) -> Self::MintTxnResult {
        let router_address: Address = router_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        let asset_in_address: Address = params.asset_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        ERC20::new(asset_in_address, self.signer.clone())
            .approve(router_address, params.amount.into())
            .call()
            .await      
            .map_err(|_| anyhow::anyhow!("Failed to mint"))?;
        MultipoolRouter::new(router_address, self.signer.clone())
            .mint_with_amount_in(
                params.pool_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.asset_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.amount.into(),
                params.shares.into(),
                params.receiver_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.deadline.into()
            )
            .call()
            .await
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("Failed to mint"))
    }

    async fn transact_burn(
        &mut self,
        router_address: &str,
        params: &crate::multipool_math::actions::settings::MintBurnTxnParams,
    ) -> Self::BurnTxnResult {
        let router_address: Address = router_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        let pool_address: Address = params.pool_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        ERC20::new(pool_address, self.signer.clone())
            .approve(router_address, params.shares.into())
            .call()
            .await      
            .map_err(|_| anyhow::anyhow!("Failed to mint"))?;
        MultipoolRouter::new(router_address, self.signer.clone())
            .burn_with_shares_in(
                pool_address,
                params.asset_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.shares.into(),
                params.amount.into(),
                params.receiver_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.deadline.into()
            )
            .call()
            .await
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("Failed to mint"))
    }

    async fn transact_burn_reversed(
        &mut self,
        router_address: &str,
        params: &crate::multipool_math::actions::settings::MintBurnTxnParams,
    ) -> Self::BurnTxnResult {
        let router_address: Address = router_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        let pool_address: Address = params.pool_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        ERC20::new(pool_address, self.signer.clone())
            .approve(router_address, params.shares.into())
            .call()
            .await      
            .map_err(|_| anyhow::anyhow!("Failed to mint"))?;
        MultipoolRouter::new(router_address, self.signer.clone())
            .burn_with_amount_out(
                pool_address,
                params.asset_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.amount.into(),
                params.shares.into(),
                params.receiver_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.deadline.into()
            )
            .call()
            .await
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("Failed to mint"))
    }

    async fn transact_swap(
        &mut self,
        router_address: &str,
        params: &crate::multipool_math::actions::settings::SwapTxnParams,
    ) -> Self::SwapTxnResult {
        let router_address: Address = router_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        let asset_in_address: Address = params.asset_in_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        ERC20::new(asset_in_address, self.signer.clone())
            .approve(router_address, params.shares.into())
            .call()
            .await      
            .map_err(|_| anyhow::anyhow!("Failed to mint"))?;
        MultipoolRouter::new(router_address, self.signer.clone())
            .swap_with_amount_in(
                params.pool_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                asset_in_address,
                params.asset_out_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.amount_in.into(),
                params.amount_out.into(),
                params.receiver_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.deadline.into()
            )
            .call()
            .await
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("Failed to mint"))
    }

    async fn transact_swap_reversed(
        &mut self,
        router_address: &str,
        params: &crate::multipool_math::actions::settings::SwapTxnParams,
    ) -> Self::SwapTxnResult {
        let router_address: Address = router_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        let asset_in_address: Address = params.asset_in_address.parse().map_err(|_e| anyhow::anyhow!(""))?;
        ERC20::new(asset_in_address, self.signer.clone())
            .approve(router_address, params.shares.into())
            .call()
            .await      
            .map_err(|_| anyhow::anyhow!("Failed to mint"))?;
        MultipoolRouter::new(router_address, self.signer.clone())
            .swap_with_amount_out(
                params.pool_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                asset_in_address,
                params.asset_out_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.amount_out.into(),
                params.amount_in.into(),
                params.receiver_address.parse().map_err(|_e| anyhow::anyhow!(""))?,
                params.deadline.into()
            )
            .call()
            .await
            .map(|_| ())
            .map_err(|_| anyhow::anyhow!("Failed to mint"))
    }
}
