use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use crate::margin::MarginFunction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendPosition {
    pub cumulative_interest: Decimal,
    pub id: String,
    pub symbol: String,
    pub imf: Decimal,
    pub imf_function: MarginFunction,
    pub mark_price: Decimal,
    pub mmf: Decimal,
    pub mmf_function: MarginFunction,
    pub net_exposure_notional: Decimal,
    pub net_exposure_quantity: Decimal,
    pub net_quantity: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteBorrowLendPayload {
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum Side {
    #[default]
    Borrow,
    Lend,
}