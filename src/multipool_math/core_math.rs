use ethers::types::U256;

use crate::num::{num::Num, snum::SNum};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MpContext {
    pub total_current_usd_amount: Num,
    pub total_asset_percents: Num,
    pub curve_coef: Num,
    pub deviation_percent_limit: Num,
    pub operation_base_fee: Num,
    pub user_cashback_balance: Num,
}

impl From<(U256, U256, U256, U256, U256, U256)> for MpContext {
    fn from(v: (U256, U256, U256, U256, U256, U256)) -> Self {
        MpContext {
            total_current_usd_amount: v.0.into(),
            total_asset_percents: v.1.into(),
            curve_coef: v.2.into(),
            deviation_percent_limit: v.3.into(),
            operation_base_fee: v.4.into(),
            user_cashback_balance: v.5.into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MpAsset {
    pub quantity: Num,
    pub price: Num,
    pub collected_fees: Num,
    pub collected_cashbacks: Num,
    pub percent: Num,
}

impl From<(U256, U256, U256, U256, U256)> for MpAsset {
    fn from(v: (U256, U256, U256, U256, U256)) -> Self {
        MpAsset {
            quantity: v.0.into(),
            price: v.1.into(),
            collected_fees: v.2.into(),
            collected_cashbacks: v.3.into(),
            percent: v.4.into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MpError {
    NoCurveSolutions,
    DeviationBiggerThanLimit,
    InsufficientBurnQuantity,
}

pub struct MpContextSigned {
    pub total_current_usd_amount: SNum,
    pub total_asset_percents: SNum,
    pub curve_coef: SNum,
    pub deviation_percent_limit: SNum,
    pub operation_base_fee: SNum,
    pub user_cashback_balance: SNum,
}

pub struct MpAssetSigned {
    pub quantity: SNum,
    pub price: SNum,
    pub collected_fees: SNum,
    pub collected_cashbacks: SNum,
    pub percent: SNum,
}

impl MpAsset {
    pub fn sign(&self) -> MpAssetSigned {
        MpAssetSigned {
            quantity: self.quantity.into(),
            price: self.price.into(),
            collected_cashbacks: self.collected_cashbacks.into(),
            collected_fees: self.collected_fees.into(),
            percent: self.percent.into(),
        }
    }
}

impl MpContext {
    pub fn sign(&self) -> MpContextSigned {
        MpContextSigned {
            total_current_usd_amount: self.total_current_usd_amount.into(),
            total_asset_percents: self.total_asset_percents.into(),
            curve_coef: self.curve_coef.into(),
            deviation_percent_limit: self.deviation_percent_limit.into(),
            operation_base_fee: self.operation_base_fee.into(),
            user_cashback_balance: self.user_cashback_balance.into(),
        }
    }
}

pub fn get_utilisable_mint_quantity(
    supplied_quantity: SNum,
    asset: &MpAssetSigned,
    context: &MpContextSigned,
) -> SNum {
    let mut utilisable_quantity = SNum::ZERO;
    let bf = SNum::from("1") + context.operation_base_fee;
    let m = SNum::from("1") - asset.percent / context.total_asset_percents;
    let cp = context.curve_coef / context.deviation_percent_limit;

    {
        let dlm = context.deviation_percent_limit - m;
        let t = asset.quantity * asset.price - context.total_current_usd_amount;
        let a = (bf * dlm + cp * m) * asset.price;
        let b = dlm * (context.total_current_usd_amount * bf - supplied_quantity * asset.price)
            - (bf - cp) * t
            + cp * m * context.total_current_usd_amount;
        let c = t * supplied_quantity - dlm * context.total_current_usd_amount * supplied_quantity;

        let d = b.pow2() - SNum::from("4") * a * c;

        let cmp = -(asset.quantity * asset.price
            + context.total_current_usd_amount * (m - SNum::from("1")))
            / (m * asset.price);

        if d >= SNum::ZERO {
            let d: SNum = d.sqrt().into();
            let x1 = (-b - d) / SNum::from("2") / a;
            let x2 = (-b + d) / SNum::from("2") / a;

            if x1 > cmp && x1 > SNum::ZERO && x1 < supplied_quantity {
                utilisable_quantity = x1;
            }
            if x2 > cmp && x2 > SNum::ZERO && x2 < supplied_quantity {
                utilisable_quantity = x2;
            }
        }
    }

    {
        let dlm = context.deviation_percent_limit + m;
        let t = asset.quantity * asset.price - context.total_current_usd_amount;
        let a = (bf * dlm - cp * m) * asset.price;
        let b = dlm * (context.total_current_usd_amount * bf - supplied_quantity * asset.price)
            + (bf - cp) * t
            - cp * m * context.total_current_usd_amount;
        let c = -t * supplied_quantity - dlm * context.total_current_usd_amount * supplied_quantity;

        let d = b.pow2() - SNum::from("4") * a * c;

        let cmp = -(asset.quantity * asset.price
            + context.total_current_usd_amount * (m - SNum::from("1")))
            / (m * asset.price);

        if d >= SNum::ZERO {
            let d: SNum = d.sqrt().into();
            let x1 = (-b - d) / SNum::from("2") / a;
            let x2 = (-b + d) / SNum::from("2") / a;

            if x1 < cmp && x1 > SNum::ZERO && x1 < supplied_quantity {
                utilisable_quantity = x1;
            }
            if x2 < cmp && x2 > SNum::ZERO && x2 < supplied_quantity {
                utilisable_quantity = x2;
            }
        }
    }

    return utilisable_quantity;
}

pub fn get_suppliable_burn_quantity(
    utilisable_quantity: SNum,
    asset: &MpAssetSigned,
    context: &MpContextSigned,
) -> SNum {
    let mut suppliable_quantity = SNum::ZERO;
    let bf = SNum::from("1") + context.operation_base_fee;
    let m = SNum::from("1") - asset.percent / context.total_asset_percents;
    let cp = context.curve_coef / context.deviation_percent_limit;

    {
        let dlm = context.deviation_percent_limit - m;
        let t = asset.quantity * asset.price - context.total_current_usd_amount;
        let a = -dlm * asset.price;
        let b = ((bf * asset.price * utilisable_quantity) + context.total_current_usd_amount) * dlm
            + cp * m * asset.price * utilisable_quantity
            - t;
        let c = -bf * context.total_current_usd_amount * utilisable_quantity * dlm
            + t * utilisable_quantity * (bf - cp)
            - cp * m * context.total_current_usd_amount * utilisable_quantity;

        let cmp = (t + m * context.total_current_usd_amount) / (m * asset.price);

        let d = b.pow2() - SNum::from("4") * a * c;

        if d > SNum::ZERO {
            let d: SNum = d.sqrt().into();
            let x1 = (-b - d) / SNum::from("2") / a;
            let x2 = (-b + d) / SNum::from("2") / a;

            if (m + t / (context.total_current_usd_amount - x1 * asset.price)).abs()
                < context.deviation_percent_limit.abs()
                && x1 < cmp
            {
                suppliable_quantity = if suppliable_quantity > x1 || suppliable_quantity.is_zero() {
                    x1
                } else {
                    suppliable_quantity
                };
            }
            if (m + t / (context.total_current_usd_amount - x2 * asset.price)).abs()
                < context.deviation_percent_limit.abs()
                && x2 < cmp
            {
                suppliable_quantity = if suppliable_quantity > x2 || suppliable_quantity.is_zero() {
                    x2
                } else {
                    suppliable_quantity
                };
            }
        }
    }

    {
        let dlm = context.deviation_percent_limit + m;
        let t = asset.quantity * asset.price - context.total_current_usd_amount;
        let a = dlm * asset.price;
        let b = -((bf * asset.price * utilisable_quantity) + context.total_current_usd_amount)
            * dlm
            + cp * m * asset.price * utilisable_quantity
            - t;
        let c = bf * context.total_current_usd_amount * utilisable_quantity * dlm
            + t * utilisable_quantity * (bf - cp)
            - cp * m * context.total_current_usd_amount * utilisable_quantity;

        let cmp = (t + m * context.total_current_usd_amount) / (m * asset.price);

        let d = b.pow2() - SNum::from("4") * a * c;

        if d > SNum::ZERO {
            let d: SNum = d.sqrt().into();
            let x1 = (-b - d) / SNum::from("2") / a;
            let x2 = (-b + d) / SNum::from("2") / a;

            if (m + t / (context.total_current_usd_amount - x1 * asset.price)).abs()
                < context.deviation_percent_limit.abs()
                && x1 > cmp
            {
                suppliable_quantity = if suppliable_quantity > x1 || suppliable_quantity.is_zero() {
                    x1
                } else {
                    suppliable_quantity
                };
            }
            if (m + t / (context.total_current_usd_amount - x2 * asset.price)).abs()
                < context.deviation_percent_limit.abs()
                && x2 > cmp
            {
                suppliable_quantity = if suppliable_quantity > x2 || suppliable_quantity.is_zero() {
                    x2
                } else {
                    suppliable_quantity
                };
            }
        }
    }

    return suppliable_quantity;
}

pub fn calculate_deviation_mint(
    utilisable_quantity: Num,
    asset: &MpAsset,
    context: &MpContext,
) -> Num {
    let share: SNum = ((asset.quantity + utilisable_quantity) * asset.price
        / (context.total_current_usd_amount + utilisable_quantity * asset.price))
        .into();
    let ideal_share: SNum = (asset.percent / context.total_asset_percents).into();
    return (share - ideal_share).abs();
}

pub fn calculate_deviation_burn(
    supplied_quantity: Num,
    asset: &MpAsset,
    context: &MpContext,
) -> Num {
    let share: SNum = ((asset.quantity - supplied_quantity) * asset.price
        / (context.total_current_usd_amount - supplied_quantity * asset.price))
        .into();
    let ideal_share: SNum = (asset.percent / context.total_asset_percents).into();
    return (share - ideal_share).abs();
}

impl MpContext {
    pub fn mint_rev(
        &mut self,
        asset: &mut MpAsset,
        utilisable_quantity: Num,
    ) -> Result<Num, MpError> {
        let context = self;
        if context.total_current_usd_amount.is_zero() {
            context.total_current_usd_amount = utilisable_quantity * asset.price;
            asset.quantity += utilisable_quantity;
            return Ok(utilisable_quantity);
        }
        let supplied_quantity;
        let deviation_new = calculate_deviation_mint(utilisable_quantity, asset, context);
        let deviation_old = calculate_deviation_mint(Num::ZERO, asset, context);

        if deviation_new <= deviation_old {
            let cashback = if !deviation_old.is_zero() {
                asset.collected_cashbacks * (deviation_old - deviation_new) / deviation_old
            } else {
                Num::ZERO
            };
            asset.collected_cashbacks -= cashback;
            context.user_cashback_balance += cashback;
            supplied_quantity =
                utilisable_quantity + utilisable_quantity * context.operation_base_fee;
        } else {
            if deviation_new > context.deviation_percent_limit {
                return Err(MpError::DeviationBiggerThanLimit);
            }

            let collected_deviation_fee = context.curve_coef * deviation_new * utilisable_quantity
                / context.deviation_percent_limit
                / (context.deviation_percent_limit - deviation_new);
            asset.collected_cashbacks += collected_deviation_fee;
            supplied_quantity = utilisable_quantity
                + utilisable_quantity * context.operation_base_fee
                + collected_deviation_fee;
        }
        asset.quantity += utilisable_quantity;
        context.total_current_usd_amount += utilisable_quantity * asset.price;
        asset.collected_fees += utilisable_quantity * context.operation_base_fee;
        return Ok(supplied_quantity);
    }

    pub fn burn_rev(
        &mut self,
        asset: &mut MpAsset,
        utilisable_quantity: Num,
    ) -> Result<Num, MpError> {
        let context = self;
        if utilisable_quantity > asset.quantity {
            return Err(MpError::InsufficientBurnQuantity);
        }

        let with_fees = get_suppliable_burn_quantity(
            utilisable_quantity.into(),
            &asset.sign(),
            &context.sign(),
        )
        .abs();
        let no_fees = utilisable_quantity * (Num::from("1") + context.operation_base_fee);

        let supplied_quantity;

        let deviation_with_fees = calculate_deviation_burn(with_fees, asset, context);
        let deviation_no_fees = calculate_deviation_burn(no_fees, asset, context);
        let deviation_old = calculate_deviation_burn(Num::ZERO, asset, context);

        if deviation_no_fees <= deviation_old {
            supplied_quantity = no_fees;
            if supplied_quantity > asset.quantity {
                return Err(MpError::InsufficientBurnQuantity);
            }
            let cashback = if !deviation_old.is_zero() {
                asset.collected_cashbacks * (deviation_old - deviation_no_fees) / deviation_old
            } else {
                Num::ZERO
            };
            asset.collected_cashbacks -= cashback;
            context.user_cashback_balance += cashback;
        } else {
            supplied_quantity = with_fees;
            if supplied_quantity > asset.quantity {
                return Err(MpError::InsufficientBurnQuantity);
            }
            if deviation_with_fees > context.deviation_percent_limit {
                return Err(MpError::DeviationBiggerThanLimit);
            }
            if with_fees.is_zero() {
                return Err(MpError::NoCurveSolutions);
            }

            asset.collected_cashbacks += supplied_quantity
                - utilisable_quantity
                - utilisable_quantity * context.operation_base_fee;
        }
        asset.quantity -= supplied_quantity;
        context.total_current_usd_amount -= supplied_quantity * asset.price;
        asset.collected_fees += utilisable_quantity * context.operation_base_fee;
        return Ok(supplied_quantity);
    }

    pub fn mint(&mut self, asset: &mut MpAsset, supplied_quantity: Num) -> Result<Num, MpError> {
        let context = self;
        if context.total_current_usd_amount.is_zero() {
            context.total_current_usd_amount = supplied_quantity * asset.price;
            asset.quantity += supplied_quantity;
            return Ok(supplied_quantity);
        }
        let utilisable_quantity;

        let with_fees =
            get_utilisable_mint_quantity(supplied_quantity.into(), &asset.sign(), &context.sign())
                .abs();
        let no_fees = supplied_quantity / (Num::from("1") + context.operation_base_fee);

        let deviation_with_fees = calculate_deviation_mint(with_fees, asset, context);
        let deviation_no_fees = calculate_deviation_mint(no_fees, asset, context);
        let deviation_old = calculate_deviation_mint(Num::ZERO, asset, context);

        if deviation_no_fees <= deviation_old {
            utilisable_quantity = no_fees;
            let cashback = if !deviation_old.is_zero() {
                asset.collected_cashbacks * (deviation_old - deviation_no_fees) / deviation_old
            } else {
                Num::ZERO
            };
            asset.collected_cashbacks -= cashback;
            context.user_cashback_balance += cashback;
        } else {
            utilisable_quantity = with_fees;
            if deviation_with_fees > context.deviation_percent_limit {
                return Err(MpError::DeviationBiggerThanLimit);
            }
            if with_fees.is_zero() {
                return Err(MpError::NoCurveSolutions);
            }

            asset.collected_cashbacks += supplied_quantity
                - utilisable_quantity
                - utilisable_quantity * context.operation_base_fee;
        }
        asset.quantity += utilisable_quantity;
        context.total_current_usd_amount += utilisable_quantity * asset.price;
        asset.collected_fees += utilisable_quantity * context.operation_base_fee;
        return Ok(utilisable_quantity);
    }

    pub fn burn(&mut self, asset: &mut MpAsset, supplied_quantity: Num) -> Result<Num, MpError> {
        let context = self;
        if supplied_quantity > asset.quantity {
            return Err(MpError::InsufficientBurnQuantity);
        }

        let utilisable_quantity;

        let deviation_new = calculate_deviation_burn(supplied_quantity, asset, context);
        let deviation_old = calculate_deviation_burn(Num::ZERO, asset, context);

        if deviation_new <= deviation_old {
            let cashback = if !deviation_old.is_zero() {
                asset.collected_cashbacks * (deviation_old - deviation_new) / deviation_old
            } else {
                Num::ZERO
            };
            asset.collected_cashbacks -= cashback;
            context.user_cashback_balance += cashback;
            utilisable_quantity = supplied_quantity / (Num::from("1") + context.operation_base_fee);
        } else {
            if deviation_new > context.deviation_percent_limit {
                return Err(MpError::DeviationBiggerThanLimit);
            }

            let fee_ratio = context.curve_coef * deviation_new
                / context.deviation_percent_limit
                / (context.deviation_percent_limit - deviation_new);
            utilisable_quantity =
                supplied_quantity / (Num::from("1") + fee_ratio + context.operation_base_fee);

            asset.collected_cashbacks += supplied_quantity
                - utilisable_quantity
                - utilisable_quantity * context.operation_base_fee;
        }
        asset.quantity -= supplied_quantity;
        context.total_current_usd_amount -= supplied_quantity * asset.price;
        asset.collected_fees += utilisable_quantity * context.operation_base_fee;
        return Ok(utilisable_quantity);
    }
}
