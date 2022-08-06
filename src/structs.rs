use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CurrencyAccount {
    pub available: u128,
    pub locked: u128,
}

impl CurrencyAccount {
    pub fn new() -> Self {
        return CurrencyAccount {
            available: 0,
            locked: 0,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Currency {
    Native {
        name: String,
        denom: String,
        ticker: String,
    }, //Native(NativeCurrency),
    Cw20 {
        name: String,
        ticker: String,
        token_address: String,
    }, //Cw20(Cw20Currency),
}
