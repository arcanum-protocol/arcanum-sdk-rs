use crate::{
    multipool_math::{MpAsset, MpContext, MpError},
    num::num::Num,
};
use pretty_assertions::assert_eq;

#[test]
fn mint_with_zero_balance_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "0".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "0".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let utilisable_quantity = "10000000".into();

    let supplied_quantity = context.mint_rev(&mut asset, utilisable_quantity).unwrap();

    let result_context = MpContext {
        total_current_usd_amount: "100000000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: "10000000".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };
    let result_supplied_quantity = "10000000".into();
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
    assert_eq!(supplied_quantity, result_supplied_quantity);
}

#[test]
fn mint_with_zero_balance() {
    let mut context = MpContext {
        total_current_usd_amount: "0".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "0".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = "10000000".into();

    let quantity_out = context.mint(&mut asset, quantity_in).unwrap();

    let result_context = MpContext {
        total_current_usd_amount: "100000000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: "10000000".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };
    let result_quantity_out = "10000000".into();
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
    assert_eq!(quantity_out, result_quantity_out);
}

#[test]
fn mint_with_deviation_fee() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "50".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = "5.0051875".into();

    let quantity_out = context.mint(&mut asset, quantity_in).unwrap();

    let result_context = MpContext {
        total_current_usd_amount: "1050".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: "55".into(),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: Num::from("0.0051875") - "0.0005".into(),
        percent: "50".into(),
    };
    let result_quantity_out = "5".into();
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
    assert_eq!(quantity_out, result_quantity_out);
}

#[test]
fn mint_with_deviation_fee_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "50".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = "5".into();

    let quantity_out = context.mint_rev(&mut asset, quantity_in).unwrap();

    let result_context = MpContext {
        total_current_usd_amount: "1050".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: "55".into(),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: Num::from("0.0051875") - "0.0005".into() - 94.into(),
        percent: "50".into(),
    };
    let result_quantity_out = Num::from("5.0051875") - 94.into();
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
    assert_eq!(quantity_out, result_quantity_out);
}

#[test]
fn burn_with_deviation_fee_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "50".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = "5".into();

    let quantity_out = context.burn_rev(&mut asset, quantity_in).unwrap();

    // gives us an approximation of 1 wei
    let result_quantity_out = Num::from("5.005866126138531618") - 3934.into();

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1000") - result_quantity_out * Num::from("10"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50") - result_quantity_out,
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: result_quantity_out - "5.0005".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_with_deviation_fee() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "50".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    // using 24 decimal over contract's 18 decimal system gives us approx lower than 18 dec
    // so it's arithmetically correct
    let quantity_in = Num::from("5.005866126138531618") - 4164.into();

    let quantity_out = context.burn(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1000") - quantity_in * Num::from("10"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50") - quantity_in,
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: quantity_in - "5.0005".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_with_no_deviation_fee() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "46".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5.0005");

    let quantity_out = context.mint(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1050"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("51"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_with_no_deviation_fee_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "46".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5");

    let quantity_out = context.mint_rev(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5.0005");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1050"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("51"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_with_no_deviation_fee_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "56".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5");

    let quantity_out = context.burn_rev(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5.0005");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("949.995"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50.9995"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_with_no_deviation_fee() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let mut asset = MpAsset {
        quantity: "56".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5.0005");

    let quantity_out = context.burn(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("949.995"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "0".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50.9995"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "0".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_with_no_deviation_fee_and_cashback() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "46".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5.0005");

    let quantity_out = context.mint(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1050"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "3.571428571428571428571500".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("51"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "3.571428571428571428571500".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_with_no_deviation_fee_and_cashback_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "46".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5");

    let quantity_out = context.mint_rev(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5.0005");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1050"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "3.571428571428571428571500".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("51"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "3.571428571428571428571500".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_with_no_deviation_fee_and_cashback_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "56".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5");

    let quantity_out = context.burn_rev(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5.0005");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("949.995"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "6.139944596199629120855000".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50.9995"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "6.139944596199629120855000".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_with_no_deviation_fee_and_cashback() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "56".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5.0005");

    let quantity_out = context.burn(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("949.995"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "6.139944596199629120855000".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50.9995"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "6.139944596199629120855000".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_with_deviation_bigger_than_limit() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "20".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5.0005");

    let quantity_out = context.mint(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1050"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "8.730158730158730158730167".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("25"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "8.730158730158730158730167".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_with_deviation_bigger_than_limit_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "20".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5");

    let quantity_out = context.mint_rev(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5.0005");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1050"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "8.730158730158730158730167".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("25"),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "8.730158730158730158730167".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_with_deviation_bigger_than_limit_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "80".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5");

    let quantity_out = context.burn_rev(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5.0005");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1000") - "50.005".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "9.649085872381784465532267".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("80") - "5.0005".into(),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "9.649085872381784465532267".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_with_deviation_bigger_than_limit() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "80".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5.0005");

    let quantity_out = context.burn(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1000") - "50.005".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("11") - "9.649085872381784465532267".into(),
    };
    let result_asset = MpAsset {
        quantity: Num::from("80") - "5.0005".into(),
        price: "10".into(),
        collected_fees: "0.0005".into(),
        collected_cashbacks: "9.649085872381784465532267".into(),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_too_much() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "50".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5000.0005");

    let quantity_out = context.mint(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("24.999528912081994491366015");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1000") + result_quantity_out * Num::from("10"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("1"),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50") + result_quantity_out,
        price: "10".into(),
        collected_fees: result_quantity_out * Num::from("0.0001"),
        collected_cashbacks: Num::from("10") + quantity_in
            - result_quantity_out
            - result_quantity_out * Num::from("0.0001"),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn mint_too_much_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "50".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("24.999528912081994491366015");

    let quantity_out = context.mint_rev(&mut asset, quantity_in).unwrap();

    let result_quantity_out = Num::from("5000.000499999999998975370523");

    let result_context = MpContext {
        total_current_usd_amount: Num::from("1000") + quantity_in * Num::from("10"),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: Num::from("1"),
    };
    let result_asset = MpAsset {
        quantity: Num::from("50") + quantity_in,
        price: "10".into(),
        collected_fees: quantity_in * Num::from("0.0001"),
        collected_cashbacks: Num::from("10") + result_quantity_out
            - quantity_in
            - quantity_in * Num::from("0.0001"),
        percent: "50".into(),
    };
    assert_eq!(quantity_out, result_quantity_out);
    assert_eq!(asset, result_asset);
    assert_eq!(context, result_context);
}

#[test]
fn burn_too_much() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "80".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "80".into(),
    };

    let quantity_in = Num::from("50");

    let quantity_out = context.burn(&mut asset, quantity_in);
    assert_eq!(Err(MpError::DeviationBiggerThanLimit), quantity_out)
}

#[test]
fn burn_too_much_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "80".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "80".into(),
    };

    let quantity_in = Num::from("50");

    let quantity_out = context.burn_rev(&mut asset, quantity_in);
    assert_eq!(Err(MpError::NoCurveSolutions), quantity_out)
}

#[test]
fn mint_too_much_being_bigger_than_limit() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "80".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5000");

    let quantity_out = context.mint(&mut asset, quantity_in);
    assert_eq!(Err(MpError::DeviationBiggerThanLimit), quantity_out)
}

#[test]
fn mint_too_much_being_bigger_than_limit_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "80".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5000");

    let quantity_out = context.mint_rev(&mut asset, quantity_in);
    assert_eq!(Err(MpError::DeviationBiggerThanLimit), quantity_out)
}

#[test]
fn burn_too_much_being_bigger_than_limit_more_than_quantity() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "20".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5000");

    let quantity_out = context.burn(&mut asset, quantity_in);
    assert_eq!(Err(MpError::InsufficientBurnQuantity), quantity_out)
}

#[test]
fn burn_too_much_being_bigger_than_limit() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "20".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("10");

    let quantity_out = context.burn(&mut asset, quantity_in);
    assert_eq!(Err(MpError::DeviationBiggerThanLimit), quantity_out)
}

#[test]
fn burn_too_much_being_bigger_than_limit_more_than_quantity_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "20".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("5000");

    let quantity_out = context.burn_rev(&mut asset, quantity_in);
    assert_eq!(Err(MpError::InsufficientBurnQuantity), quantity_out)
}

#[test]
fn burn_too_much_being_bigger_than_limit_reversed() {
    let mut context = MpContext {
        total_current_usd_amount: "1000".into(),
        total_asset_percents: "100".into(),
        curve_coef: "0.0003".into(),
        deviation_percent_limit: "0.1".into(),
        operation_base_fee: "0.0001".into(),
        user_cashback_balance: "1".into(),
    };
    let mut asset = MpAsset {
        quantity: "20".into(),
        price: "10".into(),
        collected_fees: "0".into(),
        collected_cashbacks: "10".into(),
        percent: "50".into(),
    };

    let quantity_in = Num::from("10");

    let quantity_out = context.burn_rev(&mut asset, quantity_in);
    assert_eq!(Err(MpError::DeviationBiggerThanLimit), quantity_out)
}
