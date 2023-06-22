use super::adapter::{MockedAdapter, MpAdapter};
use crate::multipool_math::{MpAsset, MpContext};

#[test]
fn mint() {
    let connection = MockedAdapter {
        assets: [(
            String::from("0x345"),
            MpAsset {
                quantity: "50".into(),
                price: "10".into(),
                collected_fees: "0".into(),
                collected_cashbacks: "0".into(),
                percent: "50".into(),
            },
        )]
        .into_iter()
        .collect(),
        context: MpContext {
            total_current_usd_amount: "1000".into(),
            total_asset_percents: "100".into(),
            curve_coef: "0.0003".into(),
            deviation_percent_limit: "0.1".into(),
            operation_base_fee: "0.0001".into(),
            user_cashback_balance: "0".into(),
        },
        total_supply: "100".into(),
        current_block: "100".into(),
    };
    // send multipool transaction
    let val = connection
        .configure()
        .amount_out("10") // enter amount in or amount out
        .slippage_percent("10") // specify slippage percent
        .pool("0x123") // set multipool address
        .asset_in("0x345") // select token
        .receiver("0xME") // select token
        .blocks_to_live("10") // add deadlines (ttl)
        .fetch() // update context and assets
        .mint() // specify txn type
        .send_mint(); // send transaction via connection
    println!("{val:?}");
}
